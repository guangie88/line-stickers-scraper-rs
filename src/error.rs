// use cssparser::{self, ParseErrorKind, SourceLocation};
use cssparser::{self, SourceLocation};
use std::fmt::Debug;

// #[derive(Debug, Fail)]
// #[fail(display = "{{ kind: {:?}, location: {:?} }}", kind, location)]
// pub struct ParseError<'i, E: 'i + Debug> {
//     pub kind: ParseErrorKind<'i, E>,
//     pub location: SourceLocation,
// }

// impl<'i, E: 'i + Debug> From<cssparser::ParseError<'i, E>>
//     for ParseError<'i, E>
// {
//     fn from(e: cssparser::ParseError<'i, E>) -> ParseError<'i, E> {
//         ParseError {
//             kind: e.kind,
//             location: e.location,
//         }
//     }
// }

#[derive(Debug, Fail)]
#[fail(display = "{{ location: {:?} }}", location)]
pub struct ParseError {
    pub location: SourceLocation,
}

impl<'i, E: 'i + Debug> From<cssparser::ParseError<'i, E>> for ParseError {
    fn from(e: cssparser::ParseError<'i, E>) -> ParseError {
        ParseError {
            location: e.location,
        }
    }
}
