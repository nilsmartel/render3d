enum Token {
    Comment(String),
    Mtllib(String),
    ObjectName(String),
    V(f64, f64, f64),
    Vt(f64, f64),
    Vn(f64, f64, f64),
    Group(String),
    Usemtl(String),
    Smoothness(Option<u8>),
    Face(Option<usize>, Option<usize>, Option<usize>),
}

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::IResult;
