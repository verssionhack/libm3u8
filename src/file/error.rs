use super::tag::{self, attribute};


pub type M3u8FileResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Tag(tag::Error),
    Attribute(attribute::Error),
    UnknownEnumValue(String),
    MisMatchTag,
    MissingNonOptionAttribute(&'static str),
}


impl From<tag::Error> for Error {
    fn from(value: tag::Error) -> Self {
        Self::Tag(value)
    }
}


impl From<attribute::Error> for Error {
    fn from(value: attribute::Error) -> Self {
        Self::Attribute(value)
    }
}
