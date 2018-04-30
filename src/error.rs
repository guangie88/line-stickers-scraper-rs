use cssparser::{self, SourceLocation};

#[derive(Debug, Fail)]
#[fail(display = "location: {:?}", location)]
pub struct ParseError {
    pub location: SourceLocation,
}

impl<'i, E: 'i> From<cssparser::ParseError<'i, E>> for ParseError {
    fn from(e: cssparser::ParseError<'i, E>) -> ParseError {
        ParseError {
            location: e.location,
        }
    }
}
