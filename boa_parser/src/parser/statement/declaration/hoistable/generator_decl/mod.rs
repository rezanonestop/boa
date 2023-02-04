#[cfg(test)]
mod tests;

use crate::parser::{
    statement::declaration::hoistable::{parse_callable_declaration, CallableDeclaration},
    AllowAwait, AllowDefault, AllowYield, Cursor, ParseResult, TokenParser,
};
use boa_ast::{function::Generator, Keyword, Punctuator};
use boa_interner::Interner;
use std::io::Read;

/// Generator declaration parsing.
///
/// More information:
///  - [MDN documentation][mdn]
///  - [ECMAScript specification][spec]
///
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*
/// [spec]: https://tc39.es/ecma262/#prod-GeneratorDeclaration
#[derive(Debug, Clone, Copy)]
pub(in crate::parser) struct GeneratorDeclaration {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
    is_default: AllowDefault,
}

impl GeneratorDeclaration {
    /// Creates a new `GeneratorDeclaration` parser.
    pub(in crate::parser) fn new<Y, A, D>(allow_yield: Y, allow_await: A, is_default: D) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
        D: Into<AllowDefault>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
            is_default: is_default.into(),
        }
    }
}

impl CallableDeclaration for GeneratorDeclaration {
    fn error_context(&self) -> &'static str {
        "generator declaration"
    }
    fn is_default(&self) -> bool {
        self.is_default.0
    }
    fn name_allow_yield(&self) -> bool {
        self.allow_yield.0
    }
    fn name_allow_await(&self) -> bool {
        self.allow_await.0
    }
    fn parameters_allow_yield(&self) -> bool {
        true
    }
    fn parameters_allow_await(&self) -> bool {
        false
    }
    fn body_allow_yield(&self) -> bool {
        true
    }
    fn body_allow_await(&self) -> bool {
        false
    }
    fn parameters_yield_is_early_error(&self) -> bool {
        true
    }
}

impl<R> TokenParser<R> for GeneratorDeclaration
where
    R: Read,
{
    type Output = Generator;

    fn parse(self, cursor: &mut Cursor<R>, interner: &mut Interner) -> ParseResult<Self::Output> {
        cursor.expect(
            (Keyword::Function, false),
            "generator declaration",
            interner,
        )?;
        cursor.expect(Punctuator::Mul, "generator declaration", interner)?;

        let result = parse_callable_declaration(&self, cursor, interner)?;

        Ok(Generator::new(Some(result.0), result.1, result.2, false))
    }
}
