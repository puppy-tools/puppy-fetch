use std::fmt::Debug;

#[derive(Clone)]
pub struct SensitiveString(String);

impl Debug for SensitiveString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SensitiveString")
            .field("inner", &self.0.chars().map(|_| '*').collect::<String>())
            .finish()
    }
}

impl From<String> for SensitiveString {
    fn from(value: String) -> Self {
        SensitiveString(value)
    }
}

impl From<SensitiveString> for String {
    fn from(value: SensitiveString) -> Self {
        value.0
    }
}