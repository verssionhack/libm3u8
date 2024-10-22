use super::attribute;


pub type TagResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownTag(String),
    UnknownTagValue(String),
    Attribute(attribute::Error),
}

impl From<attribute::Error> for Error {
    fn from(value: attribute::Error) -> Self {
        Self::Attribute(value)
    }
}
