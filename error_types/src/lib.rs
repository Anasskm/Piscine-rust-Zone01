use chrono::Local;
pub use chrono::{NaiveDate, Utc};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            return Err(FormError::new(
                "first_name".to_string(),
                self.first_name.to_string(),
                "No user name".to_string(),
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                "At least 8 characters".to_string(),
            ));
        }

        let has_alphabetic = self.password.chars().any(|c| c.is_alphabetic());
        let has_numeric = self.password.chars().any(|c| c.is_numeric());
        let has_non_alphanumeric = self.password.chars().any(|c| !c.is_alphanumeric());
        if !has_alphabetic || !has_non_alphanumeric || !has_numeric {
            return Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            ));
        }
        Ok(["Valid first name", "Valid password"].to_vec())
    }
}
