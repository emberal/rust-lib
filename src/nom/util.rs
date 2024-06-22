#[cfg(feature = "nom")]
use nom::{error::Error, IResult};

#[cfg(feature = "nom")]
pub trait IntoResult<T> {
    type Error;
    fn into_result(self) -> Result<T, Self::Error>;
}

#[cfg(feature = "nom")]
impl<T, R> IntoResult<T> for IResult<R, T> {
    type Error = nom::Err<Error<R>>;
    fn into_result(self) -> Result<T, Self::Error> {
        self.map(|(_remaining, value)| value)
    }
}

#[cfg(all(test, feature = "nom"))]
mod tests {
    use super::*;
    use nom::character::complete::char as c;

    fn parse_char(input: &str) -> IResult<&str, char> {
        c('A')(input)
    }

    #[test]
    fn test_into_result() {
        let i_result = parse_char("ABC");
        assert_eq!(i_result.into_result(), Ok('A'));
    }

    #[test]
    fn test_into_result_error() {
        let i_result = parse_char("BC");
        assert_eq!(
            i_result.into_result(),
            Err(nom::Err::Error(Error::new(
                "BC",
                nom::error::ErrorKind::Char
            )))
        );
    }
}
