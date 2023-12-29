use std::{fmt, error};

#[derive(Clone, Debug)]
pub struct InvalidPermissionIDError {
    pub id: u64,
}

impl fmt::Display for InvalidPermissionIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Invalid Permission ID: {}",
            self.id
        )
    }
    
}

#[derive(Clone, Debug)]
pub struct InvalidPermissionNameError {
    pub name: String,
}

impl fmt::Display for InvalidPermissionNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Invalid Permission Name: {}",
            self.name
        )
    }
}

#[derive(Clone, Debug)]
pub struct UnexpectedTokenError {
    pub expected: String,
    pub found: String,
}

impl error::Error for UnexpectedTokenError {}
impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Unexpected Token: Expected {}, found {}",
            self.expected,
            self.found
        )
    }
}

#[derive(Clone, Debug)]
pub struct UnexpectedEnvironentError {
    pub context: String,
}

impl error::Error for UnexpectedEnvironentError {}
impl fmt::Display for UnexpectedEnvironentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Sub-Environments are not allowed within '{}'",
            self.context
        )
    }
}

#[derive(Clone, Debug)]
pub struct UnknownPropertyError {
    pub property: String,
    pub environment: String,
}

impl error::Error for UnknownPropertyError {}
impl fmt::Display for UnknownPropertyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Unknown Property '{}' for Environment '{}'",
            self.property,
            self.environment
        )
    }
}

#[derive(Clone, Debug)]
pub struct PropertyTypeError {
    pub property: String,
    pub environment: String,
    pub expected: String,
    pub found: String,
}

impl error::Error for PropertyTypeError {}
impl fmt::Display for PropertyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Property '{}' for Environment '{}' has an invalid type: Expected {}, found {}",
            self.property,
            self.environment,
            self.expected,
            self.found
        )
    }
}


#[derive(Clone, Debug)]
pub struct ProgrammerError;

impl error::Error for ProgrammerError {}
impl fmt::Display for ProgrammerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Programmer Error: This should never happen! Please file a bug report!"
        )
    }
}