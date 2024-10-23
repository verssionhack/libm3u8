mod r#error;
pub mod r#trait;
mod r#impl;
mod r#type;
pub use r#trait::*;
pub use r#type::*;
pub use r#error::*;



#[cfg(test)]
mod tests {
    use crate::file::tag::attribute::{attributevalue_matcher, Attribute, AttributeValue};

    #[test]
    fn test_attribute_matchers() {
        assert!(attributevalue_matcher::int.is_match("1234"));
        assert!(attributevalue_matcher::hex.is_match("0x1234"));
        assert!(attributevalue_matcher::float.is_match("0.1234"));
        assert!(attributevalue_matcher::string.is_match("\"1234\""));
        assert!(attributevalue_matcher::enum_string.is_match("str1"));
        assert!(attributevalue_matcher::resolution.is_match("1234x5678"));

        assert_eq!("1234".try_into(), Ok(AttributeValue::Int(1234)));
        assert_eq!("0x1234".try_into(), Ok(AttributeValue::Hex(0x1234)));
        println!("{}", AttributeValue::Hex(0x1234).to_string());
        assert_eq!("0.1234".try_into(), Ok(AttributeValue::Float(0.1234)));
        assert_eq!("\"1234\"".try_into(), Ok(AttributeValue::String("1234".to_owned())));
        assert_eq!("str1".try_into(), Ok(AttributeValue::EnumString("str1".to_owned())));
        assert_eq!("1234x5678".try_into(), Ok(AttributeValue::Resolution(1234, 5678)));
    }


    #[test]
    fn test_parse_attribute() {
        let attr_1 = "PROGRAM-ID=1";
        let attr_2 = "BANDWIDTH=800000";
        let attr_3 = "RESOLUTION=1080x608";
        let attr_4 = "CODECS=\"avc1.42e00a,mp4a.40.2\"";
        println!("{:?}", Attribute::try_from(attr_1));
        println!("{:?}", Attribute::try_from(attr_2));
        println!("{:?}", Attribute::try_from(attr_3));
        println!("{:?}", Attribute::try_from(attr_4));
    }

    #[test]
    fn test_attributevalue_funcs() {
        assert_eq!(AttributeValue::int(1234), AttributeValue::Int(1234));
        assert_eq!(AttributeValue::hex(0x1234), AttributeValue::Hex(0x1234));
        assert_eq!(AttributeValue::float(0.1234), AttributeValue::Float(0.1234));
        assert_eq!(AttributeValue::string("1234"), AttributeValue::String("1234".to_owned()));
        assert_eq!(AttributeValue::resolution(1234, 5678), AttributeValue::resolution(1234, 5678));
        assert_eq!(AttributeValue::enum_string("1234"), AttributeValue::EnumString("1234".to_owned()));
    }
}
