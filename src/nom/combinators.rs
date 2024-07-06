use {
    nom::{
        bytes::complete::take_while_m_n,
        character::complete::{char, multispace0},
        combinator::eof,
        sequence::{delimited, terminated},
        IResult, InputIter, InputLength, InputTake, Slice,
    },
    std::ops::RangeFrom,
};

// TODO generic input

/// Trim leading and trailing whitespace from the input Parser
/// - Parameters
///    - `inner`: The parser to trim
/// - Returns: A parser that trims leading and trailing whitespace from the input and then runs the value from the inner parser
pub fn trim<'a, Parser, R>(inner: Parser) -> impl FnMut(&'a str) -> IResult<&'a str, R>
where
    Parser: FnMut(&'a str) -> IResult<&'a str, R>,
{
    delimited(multispace0, inner, multispace0)
}

/// Parse a parenthesized expression. This parser will parse an expression that is surrounded by parentheses
/// and will trim the whitespace surrounding the expression.
/// - Parameters
///     - `inner`: The parser to run inside the parentheses
/// - Returns: A parser that parses a parenthesized expression
pub fn parenthesized<'a, Parser, R>(inner: Parser) -> impl FnMut(&'a str) -> IResult<&'a str, R>
where
    Parser: FnMut(&'a str) -> IResult<&'a str, R>,
{
    // TODO move trim out of here
    delimited(char('('), trim(inner), char(')'))
}

/// Take where the predicate is true and the length is exactly `n`
/// - Parameters
///   - `n`: The length of the string to take
///   - `predicate`: The predicate to call to validate the input
/// - Returns: A parser that takes `n` characters from the input
pub fn take_where<F, Input>(n: usize, predicate: F) -> impl Fn(Input) -> IResult<Input, Input>
where
    Input: InputTake + InputIter + InputLength + Slice<RangeFrom<usize>>,
    F: Fn(<Input as InputIter>::Item) -> bool + Copy,
{
    take_while_m_n(n, n, predicate)
}

pub fn exhausted<'a, Parser, R>(inner: Parser) -> impl FnMut(&'a str) -> IResult<&'a str, R>
where
    Parser: FnMut(&'a str) -> IResult<&'a str, R>,
{
    terminated(inner, eof)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{bytes::complete::take_while, sequence::tuple};

    #[test]
    fn test_trim_both_sides() {
        let input = " test ";
        let (remaining, result) =
            trim(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_trim_leading() {
        let input = " test";
        let (remaining, result) =
            trim(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_trim_trailing() {
        let input = "test ";
        let (remaining, result) =
            trim(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_trim_no_trim() {
        let input = "test";
        let (remaining, result) =
            trim(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_parenthesized() {
        let input = "(test)";
        let (remaining, result) =
            parenthesized(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_parenthesized_parse_until_end() {
        let input = "(test)";
        assert!(parenthesized(take_while(|_| true))(input).is_err());
    }

    #[test]
    fn test_take_where() {
        let input = "test";
        let (remaining, result) = take_where(4, |c: char| c.is_ascii_alphabetic())(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_take_where_not_enough() {
        let input = "tes";
        assert!(take_where(4, |c: char| c.is_ascii_alphabetic())(input).is_err());
    }

    #[test]
    fn test_take_where_too_much() {
        let input = "testing";
        assert_eq!(
            take_where(4, |c: char| c.is_ascii_alphabetic())(input),
            Ok(("ing", "test"))
        );
    }

    #[test]
    fn test_take_where_predicate_false() {
        let input = "test";
        assert!(take_where(4, |c: char| c.is_ascii_digit())(input).is_err());
    }

    #[test]
    fn test_exhausted() {
        let input = "test";
        let (remaining, result) =
            exhausted(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_exhausted_not_exhausted() {
        let input = "test ";
        assert!(exhausted(take_where(4, |c: char| c.is_ascii_alphabetic()))(input).is_err());
    }

    #[test]
    fn test_exhausted_tuple() {
        let input = "test";
        let (remaining, result) = exhausted(tuple((
            take_where(3, |c: char| c.is_ascii_alphabetic()),
            take_while(|c: char| c.is_ascii_alphabetic()),
        )))(input)
        .unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, ("tes", "t"));
    }
}
