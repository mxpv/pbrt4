use std::ops::{Deref, DerefMut};

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Param<'a> {
    pub type_and_name: &'a str,
    pub values: Vec<Token<'a>>,
}

impl<'a> Param<'a> {
    pub fn new(type_and_name: &'a str, values: Vec<Token<'a>>) -> Self {
        Self {
            type_and_name,
            values,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ParamList<'a>(Vec<Param<'a>>);

impl<'a> Deref for ParamList<'a> {
    type Target = Vec<Param<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for ParamList<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
