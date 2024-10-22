use core::panic;
use std::{collections::HashMap, fmt::{format, Display}, ops::Deref};

use hex::ToHex;

use super::{attributevalue_matcher, Attribute, AttributeList, AttributeValue};



impl TryFrom<&str> for AttributeValue {
    type Error = super::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            if attributevalue_matcher::int.is_match(value) {
                Self::Int(value.parse().unwrap())
            } else if attributevalue_matcher::hex.is_match(value) {
                Self::Hex(u64::from_str_radix(attributevalue_matcher::hex.captures(value).unwrap().name("hex").unwrap().as_str(), 16).unwrap())
            } else if attributevalue_matcher::resolution.is_match(value) {
                let captures = attributevalue_matcher::resolution.captures(value).unwrap();
                Self::Resolution(
                    captures.name("x").unwrap().as_str().parse().unwrap(),
                    captures.name("y").unwrap().as_str().parse().unwrap()
                )
            } else if attributevalue_matcher::float.is_match(value) {
                Self::Float(value.parse().unwrap())
            } else if attributevalue_matcher::string.is_match(value) {
                Self::String(attributevalue_matcher::string.captures(value).unwrap().name("string").unwrap().as_str().to_owned())
            } else if attributevalue_matcher::enum_string.is_match(value) {
                Self::EnumString(value.to_string())
            } else {
                Err(super::Error::ParseAttributeValue {
                    attribute_value: value.to_owned()
                })?
            }
        )
    }
}


impl TryFrom<&str> for Attribute {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            if let Some((name, attr)) = value.split_once('=') {
                Attribute {
                    name: name.to_owned(),
                    value: attr.try_into()?,
                }
            } else {
                Err(super::Error::ParseAttribute {
                    attribute: value.to_owned()
                })?
            }
        )
    }
}


impl TryFrom<&str> for AttributeList {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut attribute_list = AttributeList::default();


        let mut temp_sp = String::new();

        let string_re: regex::Regex = regex::Regex::new("^.*=\"[^\\\\]*\"$").unwrap();

        for sp in value.split(',') {
            if sp.contains("=\"") && !string_re.is_match(&sp) {
                temp_sp.push_str(sp);
            } else if temp_sp.len() > 0 {
                temp_sp.push_str(",");
                temp_sp.push_str(sp);
                if string_re.is_match(&temp_sp) {
                    let attribute: Attribute = temp_sp.as_str().try_into()?;
                    attribute_list.insert(attribute);
                    temp_sp.clear();
                }
            } else {
                let attribute: Attribute = sp.try_into()?;
                attribute_list.insert(attribute);
            }
        }

        Ok(attribute_list)
    }
}

impl AttributeValue {
    pub fn int(value: u64) -> Self {
        Self::Int(value)
    }
    pub fn hex(value: u64) -> Self {
        Self::Hex(value)
    }
    pub fn float(value: f64) -> Self {
        Self::Float(value)
    }
    pub fn resolution(x: u64, y: u64) -> Self {
        Self::Resolution(x, y)
    }
    pub fn string(value: &str) -> Self {
        Self::String(value.to_owned())
    }
    pub fn enum_string(value: &str) -> Self {
        Self::EnumString(value.to_owned())
    }

    pub fn unwrap_int(self: &Self) -> u64 {
        if let Self::Int(value) = self {
            *value
        } else {
            panic!("{:?}", self);
        }
    }

    pub fn unwrap_hex(self: &Self) -> u64 {
        if let Self::Hex(value) = self {
            *value
        } else {
            panic!("{:?}", self);
        }
    }

    pub fn unwrap_float(self: &Self) -> f64 {
        if let Self::Float(value) = self {
            *value
        } else {
            panic!("{:?}", self);
        }
    }

    pub fn unwrap_resolution(self: &Self) -> (u64, u64) {
        if let Self::Resolution(x, y) = self {
            (*x, *y)
        } else {
            panic!("{:?}", self);
        }
    }

    pub fn unwrap_string(self: &Self) -> &str {
        if let Self::String(value) = self {
            value
        } else {
            panic!("{:?}", self);
        }
    }

    pub fn unwrap_enum_string(self: &Self) -> &str {
        if let Self::EnumString(value) = self {
            value
        } else {
            panic!("{:?}", self);
        }
    }
}

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        match self {
            Self::Int(v) => {
                v.to_string()
            }
            Self::Hex(v) => {
                format!("0x{}", v.to_le_bytes().encode_hex::<String>())
            }
            Self::Float(v) => {
                v.to_string()
            }
            Self::Resolution(x, y) => {
                format!("{x}x{y}")
            }
            Self::String(v) => {
                format!("\"{v}\"")
            }
            Self::EnumString(v) => {
                v.to_string()
            }
        }
    }
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        format!("{}={}", &self.name, self.value.to_string())
    }
}

impl Attribute {
    pub fn new(name: &str, value: AttributeValue) -> Self {
        Self {
            name: name.to_owned(), 
            value
        }
    }
}

impl ToString for AttributeList {
    fn to_string(&self) -> String {
        self._map.iter().fold(String::new(), |mut s, (_, v)| {
            if s.len() > 0 {
                s.push_str(",");
            }
            s.push_str(&v.to_string());
            s
        })
    }
}

impl AttributeList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&AttributeValue> {
        self._map.get(key)
    }

    pub fn insert(&mut self, attribute: Attribute) -> Option<AttributeValue> {
        self._map.insert(attribute.name, attribute.value)
    }
}
