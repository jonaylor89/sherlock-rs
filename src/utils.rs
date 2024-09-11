use std::collections::HashMap;

///
/// Creates username variants by replacing the variant symbol with check symbols.
///
/// # Arguments
/// * `usernames` - The usernames to create variants for.
///
/// # Returns
/// The username variants.
///
/// # Example
/// ```
/// use sherlock::utils::create_username_variants;
///
/// let usernames = vec![
///    String::from("user{?}name"),
///    String::from("another{?}user"),
///   String::from("test{?}user"),
/// ];
///
/// let variants = create_username_variants(&usernames);
///
/// assert_eq!(variants, vec![
///   "user_name",
///   "user-name",
///   "user.name",
///   "another_user",
///   "another-user",
///   "another.user",
///   "test_user",
///   "test-user",
///   "test.user",
/// ]);
/// ```
#[must_use]
pub fn create_username_variants(usernames: &[String]) -> Vec<String> {
    let variant_symbol = "{?}";
    let check_symbols = ["_", "-", "."];
    let variants = usernames
        .iter()
        .flat_map(|username| {
            if !username.contains(variant_symbol) {
                return vec![username.clone()];
            }

            check_symbols
                .iter()
                .map(|symbol| username.replace(variant_symbol, symbol))
                .collect::<Vec<String>>()
        })
        .collect();

    variants
}

pub trait Interpolatable {
    fn interpolate(&self, text: &str) -> Self;
}

impl Interpolatable for String {
    ///
    /// Interpolates the given text into the receiver.
    ///
    /// # Arguments
    /// * `text` - The text to interpolate.
    ///
    /// # Returns
    /// The interpolated text.
    ///
    /// # Example
    /// ```
    /// use sherlock::utils::Interpolatable;
    ///
    /// let string = "value is '{}'".to_string();
    /// let interpolated_string = string.interpolate("test".into());
    ///
    /// assert_eq!(interpolated_string, "value is 'test'");
    /// ```
    fn interpolate(&self, text: &str) -> Self {
        self.replace("{}", text)
    }
}

impl<T: Interpolatable> Interpolatable for Vec<T> {
    /// Interpolates the given text into the receiver.
    ///
    /// # Arguments
    /// * `text` - The text to interpolate.
    ///
    /// # Returns
    /// The interpolated text.
    ///
    /// # Example
    /// ```
    /// use sherlock::utils::Interpolatable;
    ///
    /// let vec = vec!["value is '{}'".to_string()];
    /// let interpolated_vec = vec.interpolate("test");
    ///
    /// assert_eq!(interpolated_vec[0], "value is 'test'");
    /// ```
    fn interpolate(&self, text: &str) -> Self {
        self.iter().map(|item| item.interpolate(text)).collect()
    }
}

impl<T: Interpolatable> Interpolatable for HashMap<String, T> {
    /// Interpolates the given text into the receiver.
    ///
    /// # Arguments
    /// * `text` - The text to interpolate.
    ///
    /// # Returns
    /// The interpolated text.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use sherlock::utils::Interpolatable;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), "value is '{}'".to_string());
    /// let interpolated_map = map.interpolate("test");
    ///
    /// assert_eq!(interpolated_map.get("key").unwrap(), "value is 'test'");
    /// ```
    fn interpolate(&self, text: &str) -> Self {
        self.iter()
            .map(|(key, value)| (key.clone(), value.interpolate(text)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_username_variants() {
        let usernames = vec![
            String::from("user{?}name"),
            String::from("another{?}user"),
            String::from("test{?}user"),
        ];
        let expected = vec![
            "user_name",
            "user-name",
            "user.name",
            "another_user",
            "another-user",
            "another.user",
            "test_user",
            "test-user",
            "test.user",
        ];
        let result = create_username_variants(&usernames);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_username_variants_no_symbol() {
        let usernames = vec![
            String::from("username"),
            String::from("anotheruser"),
            String::from("testuser"),
        ];
        let expected = vec!["username", "anotheruser", "testuser"];
        let result = create_username_variants(&usernames);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_username_variants_empty() {
        let usernames: Vec<String> = vec![];
        let expected: Vec<String> = vec![];
        let result = create_username_variants(&usernames);
        assert_eq!(result, expected);
    }
}
