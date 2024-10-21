#![allow(dead_code)]
use std::fmt;

pub enum Color{
    CWhite,
    CBlack,
    CRed,
    CGreen,
    CBlue,
    CYellow,
    BWhite,
    BBlack,
    BRed,
    BGreen,
    BBlue,
    BYellow,
    Reset
}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::CWhite => write!(f, "\x1b[97m"),
            Color::CBlack => write!(f, "\x1b[30m"),
            Color::CRed => write!(f, "\x1b[91m"),
            Color::CGreen => write!(f, "\x1b[32m"),
            Color::CBlue => write!(f, "\x1b[34m"),
            Color::CYellow => write!(f, "\x1b[93m"),
            Color::BWhite => write!(f, "\x1b[107m"),
            Color::BBlack => write!(f, "\x1b[40m"),
            Color::BRed => write!(f, "\x1b[101m"),
            Color::BGreen => write!(f, "\x1b[42m"),
            Color::BBlue => write!(f, "\x1b[44m"),
            Color::BYellow => write!(f, "\x1b[103m"),
            Color::Reset => write!(f, "\x1b[0m"),
        }
    }
}
