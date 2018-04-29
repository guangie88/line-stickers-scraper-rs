use cssparser::{self, ParseErrorKind, SourceLocation};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Fail)]
pub struct ParseError<'i, E: 'i> {
    pub kind: ParseErrorKind<'i, E>,
    pub location: SourceLocation,
}

impl<'i, T: 'i + Debug> Display for ParseError<'i, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{{ kind: {:?}, location: {:?} }}",
            self.kind, self.location
        )
    }
}

impl<'i, Eq: 'i> From<cssparser::ParseError<'i, Eq>> for ParseError<'i, Eq> {
    fn from(e: cssparser::ParseError<'i, Eq>) -> ParseError<'i, Eq> {
        ParseError {
            kind: e.kind,
            location: e.location,
        }
    }
}
