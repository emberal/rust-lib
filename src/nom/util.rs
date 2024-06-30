use {
    crate::traits::IntoResult,
    nom::{error::Error, IResult},
};

impl<T, R> IntoResult<T> for IResult<R, T> {
    type Error = nom::Err<Error<R>>;
    fn into_result(self) -> Result<T, Self::Error> {
        self.map(|(_remaining, value)| value)
    }
}

#[cfg(test)]
mod tests {
    use nom::character::complete::char as c;

    use super::*;

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
