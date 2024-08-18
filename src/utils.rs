use std::collections::HashMap;

pub fn create_username_variants(usernames: &Vec<String>) -> Vec<String> {
    let variant_symbol = "{?}";
    let check_symbols = vec!["_", "-", "."];
    let variants = usernames
        .iter()
        .map(|username| {
            check_symbols
                .iter()
                .map(|symbol| username.replace(variant_symbol, symbol))
                .collect::<Vec<String>>()
        })
        .flatten()
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
    /// let string = "value is '{}'";
    /// let interpolated_string = string.interpolate("test");
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
    /// let vec = vec!["value is '{}'"];
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
