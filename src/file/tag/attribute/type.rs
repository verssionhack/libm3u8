use std::{collections::HashMap, ops::{Deref, DerefMut}};


pub mod attributes {
    pub const METHOD: &'static str = "METHOD";
    pub const URI: &'static str = "URI";
    pub const IV: &'static str = "IV";
    pub const KEYFORMAT: &'static str = "KEYFORMAT";
    pub const KEYFORMATVERSIONS: &'static str = "KEYFORMATVERSIONS";

    pub const BYTERANGE: &'static str = "BYTERANGE";
    pub const ID: &'static str = "ID";
    pub const CLASS: &'static str = "CLASS";
    pub const START_DATE: &'static str = "START-DATE";
    pub const END_DATE: &'static str = "END-DATE";
    
    pub const DURATION: &'static str = "DURATION";
    pub const PLANNED_DURATION: &'static str = "PLANNED-DURATION";
    pub const SCTE35_CMD: &'static str = "SCTE35-CMD";
    pub const SCTE35_OUT: &'static str = "SCTE35-OUT";
    pub const SCTE35_IN: &'static str = "SCTE35-IN";

    pub const END_ON_NEXT: &'static str = "END-ON-NEXT";
    pub const TYPE: &'static str = "TYPE";
    pub const GROUP_ID: &'static str = "GROUP-ID";
    pub const LANGUAGE: &'static str = "LANGUAGE";
    pub const ASSOC_LANGUAGE: &'static str = "ASSOC-LANGUAGE";

    pub const NAME: &'static str = "NAME";
    pub const DEFAULT: &'static str = "DEFAULT";
    pub const AUTOSELECT: &'static str = "AUTOSELECT";
    pub const FORCED: &'static str = "FORCED";
    pub const INSTREAM_ID: &'static str = "INSTREAM-ID";

    pub const CHARACTERISTICS: &'static str = "CHARACTERISTICS";
    pub const CHANNELS: &'static str = "CHANNELS";
    pub const BANDWIDTH: &'static str = "BANDWIDTH";
    pub const AVERAGE_BANDWIDTH: &'static str = "AVERAGE-BANDWIDTH";
    pub const CODECS: &'static str = "CODECS";

    pub const RESOLUTION: &'static str = "RESOLUTION";
    pub const FRAME_RATE: &'static str = "FRAME-RATE";
    pub const HDCP_LEVEL: &'static str = "HDCP-LEVEL";
    pub const AUDIO: &'static str = "AUDIO";
    pub const VIDEO: &'static str = "VIDEO";

    pub const SUBTITLES: &'static str = "SUBTITLES";
    pub const CLOSED_CAPTIONS: &'static str = "CLOSED-CAPTIONS";
    pub const DATA_ID: &'static str = "DATA-ID";
    pub const VALUE: &'static str = "VALUE";
    pub const TIME_OFFSET: &'static str = "TIME-OFFSET";

    pub const PRECISE: &'static str = "PRECISE";
    pub const VIDEO_RANGE: &'static str = "VIDEO-RANGE";
}


pub mod attributevalue_matcher {
    use regex::Regex;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref int: Regex = Regex::new(r"^\d+$").unwrap();
        pub static ref hex: Regex = Regex::new("^0[xX](?<hex>[0-9a-fA-F]+)$").unwrap();
        pub static ref float: Regex = Regex::new("^-?[0-9]+\\.[0-9]+$").unwrap();
        pub static ref string: Regex = Regex::new("^\"(?<string>.*)\"$").unwrap();
        pub static ref enum_string: Regex = Regex::new(r#"^[^",\s]+$"#).unwrap();
        pub static ref resolution: Regex = Regex::new("^(?<x>[0-9]+)x(?<y>[0-9]+)$").unwrap();
    }
}


#[derive(Debug, PartialEq)]
pub enum AttributeValue {
    Int(u64),
    Hex(u64),
    Float(f64),
    Resolution(u64, u64),
    String(String),
    EnumString(String),
}


#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Default)]
pub struct AttributeList {
    pub(super) _map: HashMap<String, AttributeValue>,
}

impl Deref for AttributeList {
    type Target = HashMap<String, AttributeValue>;

    fn deref(&self) -> &Self::Target {
        &self._map
    }
}

impl DerefMut for AttributeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._map
    }
}
