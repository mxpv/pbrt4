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
    Texture,
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
            "texture" => ParamType::Texture,
            _ => return Err(Error::InvalidParamType),
        };

        Ok(ty)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Spectrum {
    //  "rgb L" [ r g b ]
    Rgb([f32; 3]),
    // "blackbody L" 3000
    Blackbody(i32),
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
            ParamType::Integer | ParamType::Blackbody => Values::Integers(Vec::new()),
            ParamType::String | ParamType::Texture => Values::Strings(Vec::new()),
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

    pub fn as_spectrum(&self) -> Option<Spectrum> {
        let res = match self.ty {
            // TODO: should we return an error if parse failed?
            ParamType::Rgb => match self.as_floats().and_then(|f| f.try_into().ok()) {
                Some(rgb) => Spectrum::Rgb(rgb),
                None => return None,
            },
            ParamType::Blackbody => match self.as_integers().and_then(|s| s.first()).copied() {
                Some(val) => Spectrum::Blackbody(val),
                None => return None,
            },
            _ => return None,
        };

        Some(res)
    }

    pub fn as_rgb(&self) -> Option<[f32; 3]> {
        match self.ty {
            ParamType::Rgb => self.as_floats().and_then(|f| f.try_into().ok()),
            _ => None,
        }
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

    pub fn float(&self, name: &str, default: f32) -> f32 {
        self.floats(name)
            .and_then(|floats| floats.first().copied())
            .unwrap_or(default)
    }

    pub fn integer(&self, name: &str, default: i32) -> i32 {
        self.integers(name)
            .and_then(|ints| ints.first().copied())
            .unwrap_or(default)
    }

    pub fn string(&self, name: &str) -> Option<&str> {
        self.strings(name).and_then(|strs| strs.first().copied())
    }

    pub fn boolean(&self, name: &str, default: bool) -> bool {
        self.booleans(name)
            .and_then(|booleans| booleans.first().copied())
            .unwrap_or(default)
    }

    pub fn extend(&mut self, other: &ParamList<'a>) {
        for (k, v) in &other.0 {
            self.0.insert(k, v.clone());
        }
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

        assert!(matches!(list.add(param), Err(Error::DuplicatedParamName)));
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

    #[test]
    fn parse_blackbody() -> Result<()> {
        let mut param = Param::new("blackbody I")?;
        param.add_token(Token::new("5500"))?;

        let i = param.as_spectrum().unwrap();

        assert!(matches!(i, Spectrum::Blackbody(5500)));
        Ok(())
    }

    #[test]
    fn parse_rgb() -> Result<()> {
        let mut param = Param::new("rgb L")?;
        param.add_token(Token::new("7"))?;
        param.add_token(Token::new("0"))?;
        param.add_token(Token::new("7"))?;

        let i = param.as_spectrum().unwrap();

        assert!(matches!(i, Spectrum::Rgb(_)));
        Ok(())
    }

    #[test]
    fn parse_texture() -> Result<()> {
        let mut param = Param::new("texture test")?;
        param.add_token(Token::new("\"float:textures/Fabric - Chaise longue\""))?;

        let value = param.as_strings().unwrap().first().unwrap().to_owned();
        assert_eq!(value, "float:textures/Fabric - Chaise longue");
        Ok(())
    }
}
