//! Tiny arithmetic-expression evaluator — a domain starter showing the
//! template's test shapes applied to a real parser: unit tests per
//! construct, property tests for panic-freedom and round-tripping, and
//! a `Result`-based error type instead of panics.
//!
//! Standalone workspace of its own (an empty `[workspace]` table — the
//! `fuzz/` detachment pattern), *not* a member of the RustForge
//! workspace: it adds nothing to the template's default
//! `cargo test --workspace`. See `docs/domain-starters.md`.
//!
//! `unsafe_code`/`missing_docs` lint policy: mirrors the RustForge
//! workspace's `[lints]` table via the crate attributes below.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Everything that can go wrong evaluating an expression. Every failure
/// mode is a variant — callers never see a panic.
#[derive(Debug, PartialEq, Eq)]
pub enum EvalError {
    /// The input contained a character the tokenizer doesn't accept.
    UnexpectedChar(char),
    /// A numeric literal too large for `i64`.
    NumberOverflow,
    /// The expression ended mid-parse (e.g. `"1 +"`).
    UnexpectedEnd,
    /// A `)` without a matching `(`, or trailing tokens after a
    /// complete expression (e.g. `"1 + 2)"`, `"1 2"`).
    UnbalancedParen,
    /// Division by zero.
    DivisionByZero,
    /// Overflow during `+`, `-`, `*`, or unary `-`.
    ArithmeticOverflow,
    /// A token appeared where the grammar doesn't allow one
    /// (e.g. `"* 3"`).
    InvalidSyntax,
}

/// Evaluates an arithmetic expression over `i64`.
///
/// Supported: integer literals, `+`, `-`, `*`, `/` (integer division,
/// truncating toward zero), unary minus, and parentheses. Division by
/// zero and every overflow path return [`EvalError`] rather than
/// panicking — the property tests below verify that for arbitrary
/// input.
///
/// # Examples
///
/// ```rust
/// use parser_starter::{eval, EvalError};
///
/// assert_eq!(eval("1 + 2 * 3"), Ok(7));
/// assert_eq!(eval("-(1 + 2)"), Ok(-3));
/// assert_eq!(eval("10 / 3"), Ok(3));
/// assert_eq!(eval("1 / 0"), Err(EvalError::DivisionByZero));
/// ```
pub fn eval(input: &str) -> Result<i64, EvalError> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(&tokens);
    let value = parser.parse_expr()?;
    if parser.pos != tokens.len() {
        return Err(EvalError::UnbalancedParen);
    }
    Ok(value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Num(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, EvalError> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        let token = match c {
            ' ' | '\t' | '\n' | '\r' => {
                i += 1;
                continue;
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            c if c.is_ascii_digit() => {
                let start = i;
                while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                    i += 1;
                }
                let text = &input[start..i];
                let n: i64 = text.parse().map_err(|_| EvalError::NumberOverflow)?;
                tokens.push(Token::Num(n));
                continue;
            }
            other => return Err(EvalError::UnexpectedChar(other)),
        };
        tokens.push(token);
        i += 1;
    }
    Ok(tokens)
}

/// Recursive-descent parser with the classic precedence grammar:
/// `expr = term (('+'|'-') term)*`, `term = factor (('*'|'/') factor)*`,
/// `factor = '-' factor | number | '(' expr ')'`.
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<Token> {
        let token = self.peek();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn parse_expr(&mut self) -> Result<i64, EvalError> {
        let mut value = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.bump();
                    value = value
                        .checked_add(self.parse_term()?)
                        .ok_or(EvalError::ArithmeticOverflow)?;
                }
                Some(Token::Minus) => {
                    self.bump();
                    value = value
                        .checked_sub(self.parse_term()?)
                        .ok_or(EvalError::ArithmeticOverflow)?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_term(&mut self) -> Result<i64, EvalError> {
        let mut value = self.parse_factor()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.bump();
                    value = value
                        .checked_mul(self.parse_factor()?)
                        .ok_or(EvalError::ArithmeticOverflow)?;
                }
                Some(Token::Slash) => {
                    self.bump();
                    let divisor = self.parse_factor()?;
                    if divisor == 0 {
                        return Err(EvalError::DivisionByZero);
                    }
                    value = value
                        .checked_div(divisor)
                        .ok_or(EvalError::ArithmeticOverflow)?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_factor(&mut self) -> Result<i64, EvalError> {
        match self.bump() {
            Some(Token::Minus) => self
                .parse_factor()?
                .checked_neg()
                .ok_or(EvalError::ArithmeticOverflow),
            Some(Token::Num(n)) => Ok(n),
            Some(Token::LParen) => {
                let value = self.parse_expr()?;
                match self.bump() {
                    Some(Token::RParen) => Ok(value),
                    Some(_) => Err(EvalError::UnbalancedParen),
                    None => Err(EvalError::UnexpectedEnd),
                }
            }
            Some(_) => Err(EvalError::InvalidSyntax),
            None => Err(EvalError::UnexpectedEnd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{eval, EvalError};

    #[test]
    fn evaluates_precedence_correctly() {
        assert_eq!(eval("1 + 2 * 3"), Ok(7));
        assert_eq!(eval("2 * 3 + 1"), Ok(7));
        assert_eq!(eval("10 - 2 - 3"), Ok(5)); // left-associative
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(eval("(1 + 2) * 3"), Ok(9));
        assert_eq!(eval("((2))"), Ok(2));
    }

    #[test]
    fn unary_minus_is_supported() {
        assert_eq!(eval("-3"), Ok(-3));
        assert_eq!(eval("--3"), Ok(3));
        assert_eq!(eval("-(1 + 2)"), Ok(-3));
    }

    #[test]
    fn integer_division_truncates() {
        assert_eq!(eval("10 / 3"), Ok(3));
        assert_eq!(eval("-10 / 3"), Ok(-3));
    }

    #[test]
    fn whitespace_is_ignored() {
        assert_eq!(eval("  1\n+\t2  "), Ok(3));
    }

    #[test]
    fn division_by_zero_is_an_error_not_a_panic() {
        assert_eq!(eval("1 / 0"), Err(EvalError::DivisionByZero));
        assert_eq!(eval("1 / (2 - 2)"), Err(EvalError::DivisionByZero));
    }

    #[test]
    fn malformed_input_returns_specific_errors() {
        assert_eq!(eval("1 +"), Err(EvalError::UnexpectedEnd));
        assert_eq!(eval("* 3"), Err(EvalError::InvalidSyntax));
        assert_eq!(eval("1 2"), Err(EvalError::UnbalancedParen));
        assert_eq!(eval("(1 + 2"), Err(EvalError::UnexpectedEnd));
        assert_eq!(eval("1 + 2)"), Err(EvalError::UnbalancedParen));
        assert_eq!(eval("1 + $"), Err(EvalError::UnexpectedChar('$')));
        assert_eq!(eval("99999999999999999999"), Err(EvalError::NumberOverflow));
    }

    #[test]
    fn overflow_is_an_error_not_a_panic() {
        assert_eq!(
            eval("9223372036854775807 + 1"),
            Err(EvalError::ArithmeticOverflow)
        );
    }
}
