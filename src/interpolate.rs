use std::collections::HashMap;

pub trait Interpolatable {
    fn interpolate(&self, text: &str) -> Self;
}

impl Interpolatable for String {
    fn interpolate(&self, text: &str) -> Self {
        self.replace("{}", text)
    }
}

impl<T: Interpolatable> Interpolatable for Vec<T> {
    fn interpolate(&self, text: &str) -> Self {
        self.iter().map(|item| item.interpolate(text)).collect()
    }
}

impl<T: Interpolatable> Interpolatable for HashMap<String, T> {
    fn interpolate(&self, text: &str) -> Self {
        self.iter()
            .map(|(key, value)| (key.clone(), value.interpolate(text)))
            .collect()
    }
}
