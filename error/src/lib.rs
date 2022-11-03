use thiserror::Error;
pub struct Dolphin {
    pub name: String,
    pub age: u8,
    pub hungry: bool,
}

// As a reminder, here are the 5 Guidelines for creating an error type:
// (1) Use an `enum` for your error type
// (2) Your error conditions should be enum variants grouped in as few enums as makes sense
// (3) Don't expose error types other than your own (not going to be a problem for this exercise)
// (4) Make your enum non-exhaustive
// (5) Implement the Debug, Display, and Error traits
// (5b) You can use thiserror's `Error` macro to derive the Display and Error traits
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DolphinError {
    #[error("Error for long name")]
    LongName,
    #[error("Error for age")]
    TooYoung,
    #[error("Error for hungry")]
    Hungry,
}

impl Dolphin {
    pub fn say_your_name(&self) -> Result<String, DolphinError> {
        if self.name.len() > 10 {
            Err(DolphinError::LongName)
        } else {
            Ok(format!("Hi, my name is {} and I'm a Dolphin!", self.name))
        }
    }
pub fn flip(&self) -> Result<String, DolphinError> {
        if self.age < 4 {
            Err(DolphinError::TooYoung)
        } else {
            Ok(format!("Yippee, I'm doing a flip!"))
        }
    }
    pub fn shake_hands(&self) -> Result<String, DolphinError> {
        if self.hungry {
            Err(DolphinError::Hungry)
        } else {
            Ok(format!("Nice to meet you, let's shake hands!"))
        }
    }
}