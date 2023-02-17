use crate::vec2::Vec2;
use crate::vec3::Vec3;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    error::ParseError,
    number::complete::double,
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::HashMap;

/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    let (i, v) = nom::bytes::complete::take_while(move |c| chars.contains(c))(i)?;

    // Check if we have seen a #
    match tag::<&str, &str, E>("#")(i) {
        Ok((i, _)) => {
            let (i, _) = nom::bytes::complete::take_while(move |c| c != '\n')(i)?;
            sp(i)
        }
        Err(_) => Ok((i, v)),
    }
}

fn parse_string_empty<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::take_while(move |c| c != '"')(i)
}

fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::take_while1(move |c| c != '"')(i)
}

fn parse_string_sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    nom::bytes::complete::take_while1(move |c| c != ' ')(i)
}

#[derive(Debug, Clone)]
pub struct Blackbody {
    pub temperature: f64,
    pub scale: f64,
}

#[derive(Debug, Clone)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB {
    pub fn color(v: f64) -> RGB {
        RGB { r: v, g: v, b: v }
    }
}

/// PBRT spectrum type
#[derive(Debug, Clone)]
pub enum Spectrum {
    RGB(RGB),
    Blackbody(Blackbody),
    Texture(String),
    Spectrum(String),
    Mapname(String),
}

pub enum BSDFFloat {
    Texture(String),
    Float(f64),
}

// Contain the list of parameter type
// some type are on the same one to avoid unecessary
// repetition in the code below
#[derive(Debug, Clone)]
pub enum Value {
    Integer(Vec<i64>),
    Float(Vec<f64>),
    Vec3(Vec<Vec3>),
    Vec2(Vec<Vec2>),
    String(String),
    Texture(String),
    Spectrum(String),
    RGB(RGB),
    Blackbody(Blackbody),
    Boolean(bool),
}
impl Value {
    pub fn into_integer(self) -> Vec<i64> {
        match self {
            Value::Integer(v) => v,
            _ => panic!("into_integer failed: {:?}", self),
        }
    }

    pub fn into_floats(self) -> Vec<f64> {
        match self {
            Value::Float(v) => v,
            _ => panic!("into_float failed: {:?}", self),
        }
    }

    pub fn into_float(self) -> f64 {
        match self {
            Value::Float(v) => {
                assert!(v.len() == 1);
                v[0]
            }
            _ => panic!("into_float failed: {:?}", self),
        }
    }

    pub fn into_Vec3(self) -> Vec3 {
        match self {
            Value::Vec3(v) => {
                assert!(v.len() == 1);
                v[0]
            }
            _ => panic!("into_Vec3 failed: {:?}", self),
        }
    }

    pub fn into_vector2(self) -> Vec2 {
        match self {
            Value::Vec2(v) => {
                assert!(v.len() == 1);
                v[0]
            }
            _ => panic!("into_vector2 failed: {:?}", self),
        }
    }

    pub fn into_vectors3(self) -> Vec<Vec3> {
        match self {
            Value::Vec3(v) => v,
            _ => panic!("into_Vec3 failed: {:?}", self),
        }
    }

    pub fn into_vectors2(self) -> Vec<Vec2> {
        match self {
            Value::Vec2(v) => v,
            _ => panic!("into_vector2 failed: {:?}", self),
        }
    }

    pub fn into_string(self) -> String {
        match self {
            Value::String(v) => v,
            _ => panic!("into_string failed: {:?}", self),
        }
    }

    pub fn into_bool(self) -> bool {
        match self {
            Value::Boolean(v) => v,
            _ => panic!("into_bool failed: {:?}", self),
        }
    }

    pub fn into_rgb(self) -> RGB {
        match self {
            Value::RGB(v) => v,
            _ => panic!("into_rgb failed: {:?}", self),
        }
    }

    pub fn into_spectrum(self) -> Spectrum {
        match self {
            Value::RGB(v) => Spectrum::RGB(v),
            Value::Blackbody(v) => Spectrum::Blackbody(v),
            Value::Texture(v) => Spectrum::Texture(v),
            Value::Spectrum(v) => Spectrum::Spectrum(v),
            _ => panic!("into_spectrum failed: {:?}", self),
        }
    }

    pub fn into_bsdf_float(self) -> BSDFFloat {
        match self {
            Value::Texture(v) => BSDFFloat::Texture(v),
            Value::Float(v) => {
                assert_eq!(v.len(), 1);
                BSDFFloat::Float(v[0])
            }
            _ => panic!("into_spectrum failed: {:?}", self),
        }
    }
}

pub fn parse_value_helper<'a, E: ParseError<&'a str>, O, F1, F2>(
    f1: F1,
    f2: F2,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F1: nom::Parser<&'a str, O, E>,
    F2: nom::Parser<&'a str, O, E>,
{
    nom::branch::alt((
        delimited(preceded(char('['), sp), f1, preceded(sp, char(']'))),
        f2,
    ))
}

pub fn parse_value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (String, Value), E> {
    let (i, (t, n)) = delimited(
        char('"'),
        nom::sequence::tuple((parse_string_sp, preceded(sp, parse_string))),
        char('"'),
    )(i)?;
    // dbg!(t, n);

    let (i, v) = match t {
        "integer" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    nom::multi::many0(preceded(sp, nom::character::complete::digit1)),
                    nom::multi::many0(preceded(sp, nom::character::complete::digit1)),
                ),
            )(i)?;
            let v = v.into_iter().map(|v| v.parse::<i64>().unwrap()).collect();
            (i, Value::Integer(v))
        }
        "bool" | "boolean" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    nom::branch::alt((
                        delimited(char('"'), nom::character::complete::alpha1, char('"')),
                        nom::character::complete::alpha1,
                    )),
                    nom::branch::alt((
                        delimited(char('"'), nom::character::complete::alpha1, char('"')),
                        nom::character::complete::alpha1,
                    )),
                ),
            )(i)?;

            let v = match v {
                "false" => false,
                "true" => true,
                _ => panic!("Wrong bool type: {}", v),
            };

            (i, Value::Boolean(v))
        }
        "point" | "normal" | "vector" | "Vec3" | "point3" => {
            let (i, v) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::multi::many0(preceded(sp, double)),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            assert_eq!(v.len() % 3, 0);
            let v = v
                .chunks_exact(3)
                .map(|v| Vec3::new(v[0], v[1], v[2]))
                .collect();
            (i, Value::Vec3(v))
        }
        "vector2" | "point2" => {
            let (i, v) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::multi::many0(preceded(sp, double)),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            assert_eq!(v.len() % 2, 0);
            let v = v.chunks_exact(2).map(|v| Vec2::new(v[0], v[1])).collect();
            (i, Value::Vec2(v))
        }
        "float" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    nom::multi::many0(preceded(sp, double)),
                    nom::multi::many0(preceded(sp, double)),
                ),
            )(i)?;
            (i, Value::Float(v))
        }
        "rgb" | "color" => {
            let (i, (r, g, b)) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::sequence::tuple((double, preceded(sp, double), preceded(sp, double))),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            (i, Value::RGB(RGB { r, g, b }))
        }
        "blackbody" => {
            let (i, (temperature, scale)) = preceded(
                sp,
                delimited(
                    preceded(char('['), sp),
                    nom::branch::alt((
                        nom::sequence::tuple((double, preceded(sp, double))),
                        nom::combinator::map(double, |f| (f, 1.0)),
                    )),
                    preceded(sp, char(']')),
                ),
            )(i)?;
            (i, Value::Blackbody(Blackbody { temperature, scale }))
        }
        "string" | "texture" | "spectrum" => {
            let (i, v) = preceded(
                sp,
                parse_value_helper(
                    delimited(char('"'), parse_string, char('"')),
                    delimited(char('"'), parse_string, char('"')),
                ),
            )(i)?;
            match t {
                "string" => (i, Value::String(v.to_owned())),
                "texture" => (i, Value::Texture(v.to_owned())),
                "spectrum" => (i, Value::Spectrum(v.to_owned())),
                _ => panic!("Impossible to convert str to type"),
            }
        }
        _ => panic!("{:?} not valid type", t),
    };

    Ok((i, (n.to_owned(), v)))
}

#[derive(Debug, Clone)]
pub enum Keyword {
    AttributeBegin,
    AttributeEnd,
    Identity,
    ObjectEnd,
    ReverseOrientation,
    TransformBegin,
    TransformEnd,
    WorldBegin,
    WorldEnd,
}
pub fn parse_keyword<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Keyword, E> {
    nom::branch::alt((
        nom::combinator::map(tag("AttributeBegin"), |_| Keyword::AttributeBegin),
        nom::combinator::map(tag("AttributeEnd"), |_| Keyword::AttributeEnd),
        nom::combinator::map(tag("Identity"), |_| Keyword::Identity),
        nom::combinator::map(tag("ReverseOrientation"), |_| Keyword::ReverseOrientation),
        nom::combinator::map(tag("TransformBegin"), |_| Keyword::TransformBegin),
        nom::combinator::map(tag("TransformEnd"), |_| Keyword::TransformEnd),
        nom::combinator::map(tag("WorldBegin"), |_| Keyword::WorldBegin),
        nom::combinator::map(tag("WorldEnd"), |_| Keyword::WorldEnd),
        nom::combinator::map(tag("ObjectEnd"), |_| Keyword::ObjectEnd),
    ))(i)
}

#[derive(Debug, Clone)]
pub enum NamedTokenType {
    Accelerator,
    AreaLightSource,
    Camera,
    CoordSys,
    CoordSysTransform,
    Film,
    Integrator,
    LightSource,
    MakeNamedMaterial,
    MakeNamedMedium,
    Material,
    NamedMaterial,
    Include,
    PixelFilter,
    Sampler,
    Shape,
    ObjectInstance,
    ObjectBegin,
    SurfaceIntegrator,
    VolumeIntegrator,
}
pub fn parse_named_token_type<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, NamedTokenType, E> {
    nom::branch::alt((
        nom::combinator::map(tag("Accelerator"), |_| NamedTokenType::Accelerator),
        nom::combinator::map(tag("AreaLightSource"), |_| NamedTokenType::AreaLightSource),
        nom::combinator::map(tag("Camera"), |_| NamedTokenType::Camera),
        nom::combinator::map(tag("CoordSysTransform"), |_| {
            NamedTokenType::CoordSysTransform
        }),
        nom::combinator::map(tag("CoordSys"), |_| NamedTokenType::CoordSys),
        nom::combinator::map(tag("Film"), |_| NamedTokenType::Film),
        nom::combinator::map(tag("Integrator"), |_| NamedTokenType::Integrator),
        nom::combinator::map(tag("LightSource"), |_| NamedTokenType::LightSource),
        nom::combinator::map(tag("MakeNamedMaterial"), |_| {
            NamedTokenType::MakeNamedMaterial
        }),
        nom::combinator::map(tag("MakeNamedMedium"), |_| NamedTokenType::MakeNamedMedium),
        nom::combinator::map(tag("Material"), |_| NamedTokenType::Material),
        nom::combinator::map(tag("NamedMaterial"), |_| NamedTokenType::NamedMaterial),
        nom::combinator::map(tag("Include"), |_| NamedTokenType::Include),
        nom::combinator::map(tag("PixelFilter"), |_| NamedTokenType::PixelFilter),
        nom::combinator::map(tag("Sampler"), |_| NamedTokenType::Sampler),
        nom::combinator::map(tag("Shape"), |_| NamedTokenType::Shape),
        nom::combinator::map(tag("ObjectInstance"), |_| NamedTokenType::ObjectInstance),
        nom::combinator::map(tag("ObjectBegin"), |_| NamedTokenType::ObjectBegin),
        nom::combinator::map(tag("SurfaceIntegrator"), |_| {
            NamedTokenType::SurfaceIntegrator
        }),
        nom::combinator::map(tag("VolumeIntegrator"), |_| {
            NamedTokenType::VolumeIntegrator
        }),
    ))(i)
}

#[derive(Debug, Clone)]
pub struct NamedToken {
    pub internal_type: String,
    pub values: HashMap<String, Value>,
    pub object_type: NamedTokenType,
}

pub fn parse_named_token<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, NamedToken, E> {
    let (i, object_type) = parse_named_token_type(i)?;
    let (i, internal_type) = nom::combinator::cut(preceded(
        sp,
        delimited(
            preceded(char('"'), sp),
            parse_string_empty, // Can be empty due to Material "" => None
            preceded(sp, char('"')),
        ),
    ))(i)?;

    let (i, values) = nom::combinator::cut(nom::multi::fold_many0(
        preceded(sp, parse_value),
        HashMap::new,
        |mut acc: HashMap<String, Value>, item: (String, Value)| {
            acc.insert(item.0, item.1);
            acc
        },
    ))(i)?;

    Ok((
        i,
        NamedToken {
            internal_type: internal_type.to_owned(),
            values,
            object_type,
        },
    ))
}
pub fn parse_named_token_many<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<NamedToken>, E> {
    nom::multi::fold_many1(
        preceded(sp, parse_named_token),
        Vec::new,
        |mut acc: Vec<NamedToken>, item: NamedToken| {
            acc.push(item);
            acc
        },
    )(i)
}

#[derive(Debug, Clone)]
pub enum Token {
    Transform(Vec<f64>),
    ConcatTransform(Vec<f64>),
    Texture {
        name: String,
        t: String,
        class: String,
        values: HashMap<String, Value>,
    },
    NamedToken(NamedToken),
    Keyword(Keyword),
    MediumInterface {
        inside: String,
        outside: String,
    },
    LookAt {
        eye: Vec3,
        look: Vec3,
        up: Vec3,
    },
    Scale(Vec3),
    Translate(Vec3),
    Rotate {
        angle: f64,
        axis: Vec3,
    },
    ActiveTransform(String),
}

pub fn transform<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, v) = preceded(
        preceded(tag("Transform"), sp),
        delimited(
            preceded(char('['), sp),
            nom::multi::many0(preceded(sp, double)),
            preceded(sp, char(']')),
        ),
    )(i)?;

    assert_eq!(v.len(), 16);

    Ok((i, Token::Transform(v)))
}

pub fn concat_transform<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, v) = preceded(
        preceded(tag("ConcatTransform"), sp),
        delimited(
            preceded(char('['), sp),
            nom::multi::many0(preceded(sp, double)),
            preceded(sp, char(']')),
        ),
    )(i)?;

    assert_eq!(v.len(), 16);

    Ok((i, Token::ConcatTransform(v)))
}

pub fn active_transform<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, v) = preceded(
        preceded(tag("ActiveTransform"), sp),
        nom::character::complete::alpha1,
    )(i)?;

    Ok((i, Token::ActiveTransform(v.to_owned())))
}

pub fn texture<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Texture"), sp)(i)?;

    let (i, (name, t, class)) = nom::sequence::tuple((
        delimited(char('"'), parse_string, char('"')),
        preceded(sp, delimited(char('"'), parse_string, char('"'))),
        preceded(sp, delimited(char('"'), parse_string, char('"'))),
    ))(i)?;

    // Contains all the info
    let (i, values) = nom::combinator::cut(nom::multi::fold_many0(
        preceded(sp, parse_value),
        HashMap::new,
        |mut acc: HashMap<String, Value>, item: (String, Value)| {
            acc.insert(item.0, item.1);
            acc
        },
    ))(i)?;

    Ok((
        i,
        Token::Texture {
            name: name.to_owned(),
            t: t.to_owned(),
            class: class.to_owned(),
            values,
        },
    ))
}

pub fn look_at<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("LookAt"), sp)(i)?;

    let (i, v) = nom::multi::many1(preceded(sp, nom::number::complete::double))(i)?;

    assert_eq!(v.len(), 9);

    Ok((
        i,
        Token::LookAt {
            eye: Vec3::new(v[0], v[1], v[2]),
            look: Vec3::new(v[3], v[4], v[5]),
            up: Vec3::new(v[6], v[7], v[8]),
        },
    ))
}

pub fn scale<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Scale"), sp)(i)?;

    let (i, (v0, v1, v2)) = nom::sequence::tuple((
        nom::number::complete::double,
        preceded(sp, nom::number::complete::double),
        preceded(sp, nom::number::complete::double),
    ))(i)?;

    Ok((i, Token::Scale(Vec3::new(v0, v1, v2))))
}

pub fn translate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Translate"), sp)(i)?;

    let (i, (v0, v1, v2)) = nom::sequence::tuple((
        nom::number::complete::double,
        preceded(sp, nom::number::complete::double),
        preceded(sp, nom::number::complete::double),
    ))(i)?;

    Ok((i, Token::Translate(Vec3::new(v0, v1, v2))))
}

pub fn rotate<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("Rotate"), sp)(i)?;

    let (i, (angle, v0, v1, v2)) = nom::sequence::tuple((
        nom::number::complete::double,
        preceded(sp, nom::number::complete::double),
        preceded(sp, nom::number::complete::double),
        preceded(sp, nom::number::complete::double),
    ))(i)?;

    Ok((
        i,
        Token::Rotate {
            angle,
            axis: Vec3::new(v0, v1, v2),
        },
    ))
}

pub fn medium_interface<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    let (i, _) = preceded(tag("MediumInterface"), sp)(i)?;

    let (i, (inside, outside)) = preceded(
        sp,
        nom::sequence::tuple((
            delimited(char('"'), parse_string_empty, char('"')),
            preceded(sp, delimited(char('"'), parse_string_empty, char('"'))),
        )),
    )(i)?;

    Ok((
        i,
        Token::MediumInterface {
            inside: inside.to_owned(),
            outside: outside.to_owned(),
        },
    ))
}

pub fn parse_token<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Token, E> {
    nom::branch::alt((
        nom::combinator::map(parse_named_token, |v| Token::NamedToken(v)),
        nom::combinator::map(parse_keyword, |v| Token::Keyword(v)),
        transform,
        texture,
        medium_interface,
        look_at,
        scale,
        concat_transform,
        translate,
        rotate,
        active_transform,
    ))(i)
}

pub fn parse<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<Token>, E> {
    let (i, v) = preceded(
        sp,
        nom::multi::fold_many0(
            preceded(sp, parse_token),
            Vec::new,
            |mut acc: Vec<Token>, item: Token| {
                acc.push(item);
                acc
            },
        ),
    )(i)?;

    // Skip last char
    // TODO: Make sure that it is Ok to do that
    let (i, _) = sp(i)?;

    Ok((i, v))
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    fn check_parsing<
        'a,
        T: std::fmt::Debug,
        E: nom::error::ParseError<&'a str> + std::fmt::Debug,
    >(
        res: nom::IResult<&'a str, T, E>,
        verbose: bool,
    ) {
        let (i, v) = res.expect("Error during parsing");
        if verbose {
            println!("==============================");
            println!("Parsed: {:?}", v);
            println!("==============================");
        }

        match i {
            "" => (),
            _ => panic!("Parsing is not complete: {:?}", i),
        }
    }

    #[test]
    fn transform() {
        let s = "Transform [ 0.993341 -0.0130485 -0.114467 -0 -0 0.993565 -0.11326 -0 -0.115208 -0.112506 -0.98695 -0 1.33651 -11.1523 51.6855 1]";
        check_parsing(
            crate::parser::transform::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }

    #[test]
    fn value_integer() {
        let s = "\"integer maxdepth\" [ 65 ]";
        check_parsing(
            crate::parser::parse_value::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }
    #[test]
    fn value_float() {
        let s = "\"float ywidth\" [ 1.000000 ]";
        check_parsing(
            crate::parser::parse_value::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }

    #[test]
    fn value_string() {
        let s = "\"string filename\" [ \"bathroom2.png\" ]";
        check_parsing(
            crate::parser::parse_value::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }

    #[test]
    fn statement() {
        let s = "Film \"image\" \"integer xresolution\" [ 1280 ] \"integer yresolution\" [ 720 ] \"string filename\" [ \"bathroom2.png\" ]";
        check_parsing(
            crate::parser::parse_named_token::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }

    #[test]
    fn statements() {
        let s = r#"Integrator "path" "integer maxdepth" [ 65 ] 
        Sampler "sobol" "integer pixelsamples" [ 64 ] 
        PixelFilter "triangle" "float xwidth" [ 1.000000 ] "float ywidth" [ 1.000000 ] 
        Film "image" "integer xresolution" [ 1024 ] "integer yresolution" [ 1024 ] "string filename" [ "cornell-box.png" ] 
        Camera "perspective" "float fov" [ 19.500000 ]"#;

        check_parsing(
            crate::parser::parse_named_token_many::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }

    #[test]
    fn shape() {
        let s = r#"Shape "trianglemesh" "integer indices" [ 0 1 2 0 2 3 ] "point P" [ -1 1.74846e-007 -1 -1 1.74846e-007 1 1 -1.74846e-007 1 1 -1.74846e-007 -1 ] "normal N" [ 4.37114e-008 1 1.91069e-015 4.37114e-008 1 1.91069e-015 4.37114e-008 1 1.91069e-015 4.37114e-008 1 1.91069e-015 ] "float uv" [ 0 0 1 0 1 1 0 1 ]"#;
        check_parsing(
            crate::parser::parse_named_token_many::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }

    #[test]
    fn statements2() {
        let s = r#"	MakeNamedMaterial "Light" "string type" [ "matte" ] "rgb Kd" [ 0.000000 0.000000 0.000000 ] 
        NamedMaterial "Floor" 
        Shape "trianglemesh" "integer indices" [ 0 1 2 0 2 3 ] "point P" [ -1 1.74846e-007 -1 -1 1.74846e-007 1 1 -1.74846e-007 1 1 -1.74846e-007 -1 ] "normal N" [ 4.37114e-008 1 1.91069e-015 4.37114e-008 1 1.91069e-015 4.37114e-008 1 1.91069e-015 4.37114e-008 1 1.91069e-015 ] "float uv" [ 0 0 1 0 1 1 0 1 ]"#;
        check_parsing(
            crate::parser::parse::<nom::error::VerboseError<&str>>(s),
            true,
        );
    }
}
