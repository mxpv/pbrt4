//! Directives parser.

use crate::{
    param::{Param, ParamList},
    token::{Directive, Token},
    tokenizer::Tokenizer,
    Error, Result,
};

/// Parsed directive.
#[derive(Debug, PartialEq)]
pub enum Element<'a> {
    Include(&'a str),
    Import(&'a str),
    Option(Param<'a>),
    Film {
        ty: &'a str,
        params: ParamList<'a>,
    },
    ColorSpace {
        ty: &'a str,
    },
    Camera {
        ty: &'a str,
        params: ParamList<'a>,
    },
    Sampler {
        ty: &'a str,
        params: ParamList<'a>,
    },
    Integrator {
        ty: &'a str,
        params: ParamList<'a>,
    },
    Accelerator {
        ty: &'a str,
        params: ParamList<'a>,
    },
    CoordinateSystem {
        name: &'a str,
    },
    CoordSysTransform {
        name: &'a str,
    },
    PixelFilter {
        name: &'a str,
    },
    Identity,
    /// `Translate x y z`
    Translate {
        v: [f32; 3],
    },
    /// `Scale x y z`
    Scale {
        v: [f32; 3],
    },
    /// `Rotate angle x y z`
    Rotate {
        angle: f32,
        v: [f32; 3],
    },
    /// `LookAt eye_x eye_y eye_z look_x look_y look_z up_x up_y up_z`
    LookAt {
        eye: [f32; 3],
        look_at: [f32; 3],
        up: [f32; 3],
    },
    /// `Transform m00 ... m33`
    Transform {
        m: [f32; 16],
    },
    /// `ConcatTransform m00 .. m33`
    ConcatTransform {
        m: [f32; 16],
    },
    /// `TransformTimes start end`.
    TransformTimes {
        start: f32,
        end: f32,
    },
    ActiveTransform {
        ty: &'a str,
    },
    /// `ReverseOrientation`.
    ReverseOrientation,
    /// `WorldBegin`
    WorldBegin,
    /// `AttributeBegin`
    AttributeBegin,
    /// `AttributeEnd`
    AttributeEnd,
    /// `Attribute "target" parameter-list`
    Attribute {
        target: &'a str,
        params: ParamList<'a>,
    },
    LightSource {
        ty: &'a str,
        params: ParamList<'a>,
    },
    AreaLightSource {
        ty: &'a str,
        params: ParamList<'a>,
    },
    Material {
        ty: &'a str,
        params: ParamList<'a>,
    },
    MakeNamedMaterial {
        name: &'a str,
        params: ParamList<'a>,
    },
    NamedMaterial {
        name: &'a str,
    },
    /// `Texture "name" "type" "class" [ parameter-list ]`
    Texture {
        name: &'a str,
        ty: &'a str,
        class: &'a str,
        params: ParamList<'a>,
    },
    /// `Shape "name" parameter-list`
    Shape {
        name: &'a str,
        params: ParamList<'a>,
    },
    ObjectBegin {
        name: &'a str,
    },
    ObjectEnd,
    ObjectInstance {
        name: &'a str,
    },
    MakeNamedMedium {
        name: &'a str,
        params: ParamList<'a>,
    },
    MediumInterface {
        interior: &'a str,
        exterior: &'a str,
    },
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Self {
        let tokenizer = Tokenizer::new(str);
        Self { tokenizer }
    }

    /// Parse next element.
    pub fn parse_next(&mut self) -> Result<Element<'a>> {
        let Some(next_token) = self.tokenizer.next() else {
            return Err(Error::EndOfFile);
        };

        // Check if token is directive
        let directive = next_token.directive().ok_or(Error::UnknownDirective)?;

        let element = match directive {
            Directive::Include => Element::Include(self.read_str()?),
            Directive::Import => Element::Import(self.read_str()?),
            Directive::Option => Element::Option(self.read_param()?),
            Directive::Film => Element::Film {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::ColorSpace => Element::ColorSpace {
                ty: self.read_str()?,
            },
            Directive::Camera => Element::Camera {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::Sampler => Element::Sampler {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::Integrator => Element::Integrator {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::Accelerator => Element::Accelerator {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::CoordinateSystem => Element::CoordinateSystem {
                name: self.read_str()?,
            },
            Directive::CoordSysTransform => Element::CoordSysTransform {
                name: self.read_str()?,
            },
            Directive::PixelFilter => Element::PixelFilter {
                name: self.read_str()?,
            },
            Directive::Identity => Element::Identity,
            Directive::Translate => Element::Translate {
                v: self.read_point()?,
            },
            Directive::Scale => Element::Scale {
                v: self.read_point()?,
            },
            Directive::Rotate => Element::Rotate {
                angle: self.read_float()?,
                v: self.read_point()?,
            },
            Directive::LookAt => Element::LookAt {
                eye: self.read_point()?,
                look_at: self.read_point()?,
                up: self.read_point()?,
            },
            Directive::Transform => {
                // Skip [
                self.skip_brace()?;

                let elem = Element::Transform {
                    m: self.read_matrix()?,
                };

                // Skip ]
                self.skip_brace()?;

                elem
            }
            Directive::ConcatTransform => {
                // Skip [
                self.skip_brace()?;

                let elem = Element::ConcatTransform {
                    m: self.read_matrix()?,
                };

                // Skip ]
                self.skip_brace()?;

                elem
            }
            Directive::TransformTimes => Element::TransformTimes {
                start: self.read_float()?,
                end: self.read_float()?,
            },
            Directive::ActiveTransform => Element::ActiveTransform {
                ty: self.read_str()?,
            },
            Directive::ReverseOrientation => Element::ReverseOrientation,
            Directive::WorldBegin => Element::WorldBegin,
            Directive::AttributeBegin => Element::AttributeBegin,
            Directive::AttributeEnd => Element::AttributeEnd,
            Directive::Attribute => Element::Attribute {
                target: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::LightSource => Element::LightSource {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::AreaLightSource => Element::AreaLightSource {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::Material => Element::Material {
                ty: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::MakeNamedMaterial => Element::MakeNamedMaterial {
                name: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::NamedMaterial => Element::NamedMaterial {
                name: self.read_str()?,
            },
            Directive::Texture => Element::Texture {
                name: self.read_str()?,
                ty: self.read_str()?,
                class: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::Shape => Element::Shape {
                name: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::ObjectBegin => Element::ObjectBegin {
                name: self.read_str()?,
            },
            Directive::ObjectEnd => Element::ObjectEnd,
            Directive::ObjectInstance => Element::ObjectInstance {
                name: self.read_str()?,
            },
            Directive::MakeNamedMedium => Element::MakeNamedMedium {
                name: self.read_str()?,
                params: self.read_param_list()?,
            },
            Directive::MediumInterface => Element::MediumInterface {
                interior: self.read_str()?,
                exterior: self.read_str()?,
            },
        };

        Ok(element)
    }

    fn skip_brace(&mut self) -> Result<()> {
        let Some(token) = self.tokenizer.next() else {
            return Err(Error::UnexpectedToken);
        };

        let is_open = token.is_open_brace();
        let is_close = token.is_close_brace();

        // Not a brace
        if !is_open && !is_close {
            return Err(Error::UnexpectedToken);
        }

        Ok(())
    }

    /// Read next token or return [Error::UnexpectedEnd].
    fn read_token(&mut self) -> Result<Token<'a>> {
        match self.tokenizer.next() {
            Some(token) => {
                if !token.is_valid() {
                    return Err(Error::InvalidToken);
                }

                Ok(token)
            }
            None => Err(Error::NoToken),
        }
    }

    /// Read token as `f32`.
    fn read_float(&mut self) -> Result<f32> {
        let token = self.read_token()?;
        let parsed = token.parse::<f32>()?;
        Ok(parsed)
    }

    /// Read 3 floats.
    fn read_point(&mut self) -> Result<[f32; 3]> {
        let x = self.read_float()?;
        let y = self.read_float()?;
        let z = self.read_float()?;

        Ok([x, y, z])
    }

    /// Read 16 floats.
    fn read_matrix(&mut self) -> Result<[f32; 16]> {
        let mut m = [0_f32; 16];
        for m in &mut m {
            *m = self.read_float()?;
        }
        Ok(m)
    }

    /// Read a quoted string.
    fn read_str(&mut self) -> Result<&'a str> {
        let token = self.read_token()?;
        token.unquote().ok_or(Error::InvalidString)
    }

    /// Parse a single option
    ///
    /// Valid inputs:
    /// - "integer indices" [ 0 1 2 0 2 3 ]
    /// - "float scale" [10]
    /// - "float iso" 150
    fn read_param(&mut self) -> Result<Param<'a>> {
        let type_and_name = self.read_str()?;

        let mut start = self.tokenizer.offset();
        let end;

        // Either [ or a single value.
        let value = self.read_token()?;

        if value.is_open_brace() {
            // Skip brace offset
            start = self.tokenizer.offset();

            // Read array of values
            loop {
                let value = self.read_token()?;

                if value.is_close_brace() {
                    end = self.tokenizer.offset() - 1;
                    break;
                }

                // Got directive without closing bracket token.
                if value.is_directive() {
                    return Err(Error::UnexpectedToken);
                }
            }
        } else {
            // Single value
            end = start + value.token_size() + 1;
        }

        let token = self.tokenizer.token(start, end);
        let param = Param::new(type_and_name, token.value())?;

        Ok(param)
    }

    #[inline]
    fn read_param_list(&mut self) -> Result<ParamList<'a>> {
        let mut list = ParamList::default();

        loop {
            match self.tokenizer.peek_token() {
                // Each parameter starts with a quoted string
                Some(token) if token.is_quote() => {
                    let param = self.read_param()?;
                    list.add(param)?;
                }
                // Other token, break loop
                Some(_) => break,
                // No more tokens
                None => break,
            }
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ParamType;

    use super::*;

    #[test]
    fn parse_includes() {
        let mut parser = Parser::new(
            "
Include \"geometry/car.pbrt\"
Import \"geometry/bigcar.pbrt.gz\"
        ",
        );

        let element = parser.parse_next().unwrap();
        assert!(matches!(element, Element::Include("geometry/car.pbrt")));

        let element = parser.parse_next().unwrap();
        assert!(matches!(
            element,
            Element::Import("geometry/bigcar.pbrt.gz")
        ));
    }

    #[test]
    fn parse_scale_and_rotate() {
        let mut parser = Parser::new(
            "
Scale -1 1 1
Rotate 1 0 0 1
        ",
        );

        assert!(matches!(
            parser.parse_next().unwrap(),
            Element::Scale { .. }
        ));

        assert!(matches!(
            parser.parse_next().unwrap(),
            Element::Rotate { .. }
        ));
    }

    #[test]
    fn parse_look_at() {
        let mut parser = Parser::new(
            "
        LookAt 0.322839 0.0534825 0.504299
        -0.140808 -0.162727 -0.354936
        0.0355799 0.964444 -0.261882
        ",
        );

        let element = parser.parse_next().unwrap();
        assert!(matches!(element, Element::LookAt { .. }));
    }

    #[test]
    fn parse_option() {
        let mut parser = Parser::new(
            "
Option \"string filename\" [\"foo.exr\"]
Option \"string filename\" \"foo.exr\"
        ",
        );

        let expected = Param::new("string filename", "\"foo.exr\"").unwrap();

        assert_eq!(
            parser.parse_next().unwrap(),
            Element::Option(expected.clone())
        );

        assert_eq!(parser.parse_next().unwrap(), Element::Option(expected));
    }

    #[test]
    fn parse_film() {
        let mut parser = Parser::new(
            "
Film \"rgb\"
    \"string filename\" [ \"crown.exr\" ]
    \"integer yresolution\" [ 1400 ]
    \"integer xresolution\" [ 1000 ]
    \"float iso\" 150
    \"string sensor\" \"canon_eos_5d_mkiv\"
        ",
        );

        let elem = parser.parse_next().unwrap();

        match elem {
            Element::Film { ty, params } => {
                assert_eq!(ty, "rgb");
                assert_eq!(params.len(), 5);

                let param = params.get("filename").unwrap();
                assert_eq!(param.name, "filename");
                assert_eq!(param.ty, ParamType::String);

                let param = params.get("iso").unwrap();
                assert_eq!(param.name, "iso");
                assert_eq!(param.ty, ParamType::Float)
            }
            _ => panic!("Unexpected element type"),
        }
    }

    #[test]
    fn parse_film_no_params() {
        let mut parser = Parser::new(
            "
Film \"rgb\"
LookAt 0 5.5 24 0 11 -10 0 1 0
        ",
        );

        assert!(matches!(
            parser.parse_next().unwrap(),
            Element::Film { ty: "rgb", .. }
        ));

        assert!(matches!(
            parser.parse_next().unwrap(),
            Element::LookAt { .. }
        ));
    }

    #[test]
    fn parse_transform() {
        let mut parser = Parser::new("Transform [ 1 0 0 0 0 1 0 0 0 0 1 0 3 1 -4 1 ]");
        let next = parser.parse_next().unwrap();

        assert!(matches!(next, Element::Transform { .. }));
    }

    #[test]
    fn parse_concat_transform() {
        let mut parser = Parser::new("ConcatTransform [ 1 0 0 0 0 1 0 0 0 0 1 0 3 1 -4 1 ]");
        let next = parser.parse_next().unwrap();

        assert!(matches!(next, Element::ConcatTransform { .. }));
    }
}
