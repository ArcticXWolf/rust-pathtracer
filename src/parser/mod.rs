use std::path::Path;

use nom::{
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    error::ParseError,
    number::complete::double,
    IResult,
};

pub fn parse_pbrt_file(filepath: &Path) {}

pub enum Spectrum {
    RGB(),
    Blackbody(),
    Texture(),
}

pub enum PbrtValue {
    Integer(Vec<i64>),
    Float(Vec<f64>),
    Vector2(Vec<[f64; 2]>),
    Vector3(Vec<[f64; 3]>),
    Spectrum(),
    Bool(bool),
    String(String),
}

pub enum PbrtToken {
    LookAt,
    Camera,
    Sampler,
    Integrator,
    Film,
    WorldBegin,
    LightSource,
    AttributeBegin,
    Material,
    Texture,
    Shape,
    Translate,
    AttributeEnd,
    WorldEnd,
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    let (i, v) = take_while(move |c| chars.contains(c))(i)?;

    match tag::<&str, &str, E>("#")(i) {
        Ok((i, _)) => {
            let (i, _) = take_while(move |c| c != '\n')(i)?;
            sp(i)
        }
        Err(_) => Ok((i, v)),
    }
}

fn parse_string_empty<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(move |c| c != '"')(i)
}

fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while1(move |c| c != '"')(i)
}

fn parse_string_sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while1(move |c| c != ' ')(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomics() {}
}
