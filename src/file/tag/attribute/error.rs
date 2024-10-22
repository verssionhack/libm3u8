use super::AttributeValue;


pub type FileResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseAttribute {
        attribute: String,
    },
    ParseAttributeValue {
        attribute_value: String,
    },
}

/*
impl From<::Error> for Error {
    fn from(value: ::Error) -> Self {
    }
}
*/
  
