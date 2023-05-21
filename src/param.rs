//! Parameter management.

use std::{collections::HashMap, str::FromStr};

use crate::{token::Token, Error, Result};

/// Parameter type.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParamType {
    Boolean,
    Float,
    Integer,
    Point2,
    Point3,
    Vector2,
    Vector3,
    Normal3,
    Spectrum,
    Rgb,
    Blackbody,
    String,
}

impl FromStr for ParamType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let ty = match s {
            "bool" => ParamType::Boolean,
            "integer" => ParamType::Integer,
            "float" => ParamType::Float,
            "point2" => ParamType::Point2,
            "vector2" => ParamType::Vector2,
            "point3" => ParamType::Point3,
            "vector3" => ParamType::Vector3,
            "normal3" => ParamType::Normal3,
            "spectrum" => ParamType::Spectrum,
            "rgb" => ParamType::Rgb,
            "blackbody" => ParamType::Blackbody,
            "string" => ParamType::String,
            _ => return Err(Error::InvalidParamType),
        };

        Ok(ty)
    }
}

/// Values variant.
#[derive(Debug, PartialEq, Clone)]
pub enum Values<'a> {
    Floats(Vec<f32>),
    Integers(Vec<i32>),
    Strings(Vec<&'a str>),
    Booleans(Vec<bool>),
}

/// Represents a single parsed parameter.
#[derive(Debug, PartialEq, Clone)]
pub struct Param<'a> {
    /// Parameter name.
    pub name: &'a str,
    /// Parameter type.
    pub ty: ParamType,
    /// One or more values.
    values: Values<'a>,
}

impl<'a> Param<'a> {
    pub fn new(type_and_name: &'a str) -> Result<Self> {
        // Param name is "type name"
        let mut split = type_and_name.split_whitespace();

        let ty_name = split.next().ok_or(Error::InvalidParamName)?;
        let ty = ParamType::from_str(ty_name)?;

        let name = split.next().ok_or(Error::InvalidParamName)?;

        let values = match ty {
            ParamType::Boolean => Values::Booleans(Vec::new()),
            ParamType::Integer => Values::Integers(Vec::new()),
            ParamType::String => Values::Strings(Vec::new()),
            _ => Values::Floats(Vec::new()),
        };

        Ok(Self { name, ty, values })
    }

    pub fn add_token(&mut self, token: Token<'a>) -> Result<()> {
        match &mut self.values {
            Values::Floats(ref mut floats) => {
                floats.push(token.parse()?);
            }
            Values::Integers(ref mut ints) => {
                ints.push(token.parse()?);
            }
            Values::Strings(ref mut strs) => {
                let str = token.unquote().ok_or(Error::InvalidToken)?;
                strs.push(str);
            }
            Values::Booleans(ref mut booleans) => {
                booleans.push(token.parse()?);
            }
        }

        Ok(())
    }

    pub fn as_floats(&self) -> Option<&[f32]> {
        match &self.values {
            Values::Floats(ref v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_integers(&self) -> Option<&[i32]> {
        match &self.values {
            Values::Integers(ref v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_strings(&self) -> Option<&[&str]> {
        match &self.values {
            Values::Strings(ref s) => Some(s.as_slice()),
            _ => None,
        }
    }

    pub fn as_booleans(&self) -> Option<&[bool]> {
        match &self.values {
            Values::Booleans(ref b) => Some(b.as_slice()),
            _ => None,
        }
    }
}

/// Parameters collection.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ParamList<'a>(HashMap<&'a str, Param<'a>>);

impl<'a> ParamList<'a> {
    /// Add a new parameter to the list.
    pub fn add(&mut self, param: Param<'a>) -> Result<()> {
        if self.0.insert(param.name, param).is_some() {
            return Err(Error::DuplicatedParamName);
        }

        Ok(())
    }

    /// Get parameter by name.
    pub fn get(&self, name: &str) -> Option<&Param<'a>> {
        self.0.get(name)
    }

    /// Return the number of parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` when the list is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Attempt to get parameter as a slice of `f32`.
    pub fn floats(&self, name: &str) -> Option<&[f32]> {
        self.get(name).and_then(|param| param.as_floats())
    }

    pub fn integers(&self, name: &str) -> Option<&[i32]> {
        self.get(name).and_then(|param| param.as_integers())
    }

    pub fn strings(&self, name: &str) -> Option<&[&str]> {
        self.get(name).and_then(|param| param.as_strings())
    }

    pub fn booleans(&self, name: &str) -> Option<&[bool]> {
        self.get(name).and_then(|param| param.as_booleans())
    }

    pub fn float(&self, name: &str) -> Option<f32> {
        self.floats(name).and_then(|floats| floats.first().copied())
    }

    pub fn integer(&self, name: &str) -> Option<i32> {
        self.integers(name).and_then(|ints| ints.first().copied())
    }

    pub fn string(&self, name: &str) -> Option<&str> {
        self.strings(name).and_then(|strs| strs.first().copied())
    }

    pub fn boolean(&self, name: &str) -> Option<bool> {
        self.booleans(name)
            .and_then(|booleans| booleans.first().copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_param_type() {
        assert_eq!(ParamType::from_str("bool").ok(), Some(ParamType::Boolean));

        assert_eq!(ParamType::from_str("float").ok(), Some(ParamType::Float));
        assert_eq!(
            ParamType::from_str("integer").ok(),
            Some(ParamType::Integer)
        );

        assert_eq!(ParamType::from_str("point2").ok(), Some(ParamType::Point2));
        assert_eq!(ParamType::from_str("point3").ok(), Some(ParamType::Point3));

        assert_eq!(ParamType::from_str("rgb").ok(), Some(ParamType::Rgb));
    }

    #[test]
    fn add_dup_param() {
        let mut list = ParamList::default();

        let param = Param::new("bool dup_name").unwrap();

        list.add(param.clone()).unwrap();

        assert_eq!(list.add(param), Err(Error::DuplicatedParamName));
    }

    #[test]
    fn as_ints() {
        let mut param = Param::new("integer test").unwrap();
        param.add_token(Token::new("-1")).unwrap();
        param.add_token(Token::new("0")).unwrap();
        param.add_token(Token::new("1")).unwrap();

        assert_eq!(param.as_integers(), Some([-1, 0, 1].as_slice()));
        assert_eq!(param.as_floats(), None);
    }
}
