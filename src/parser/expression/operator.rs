use super::super::types::Type;
use super::Expression;
use crate::lexer::*;
use std::io::Read;

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    LogicalAnd,
    LogicalOr,
}

macro_rules! parse_binary {
    ($tokens: ident, $next: ident, $($operator: ident -> $resulting: ident),+) => {{
        let mut lhs = Self::$next($tokens)?;

        loop {
            $(
                if $tokens.match_operator(Operator::$operator) {
                    if let Some(rhs) = Self::$next($tokens) {
                        lhs = Expression::Binary(
                            Box::new(lhs),
                            BinaryOperator::$resulting,
                            Box::new(rhs),
                        );
                        continue;
                    } else {
                        $tokens.error("expected expression");
                    }
                }
            )+
            break;
        }
        Some(lhs)
    }};
}

impl Expression {
    pub(super) fn parse_lazy_boolean<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Self> {
        parse_binary!(tokens, parse_literal, LogicalAnd -> LogicalAnd, LogicalOr -> LogicalOr)
    }

    pub(super) fn parse_typecast<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Self> {
        let mut lhs = Self::parse_lazy_boolean(tokens)?;

        while tokens.match_keyword(Keyword::As) {
            lhs = Expression::TypeCast(Box::new(lhs), Type::parse(tokens));
        }
        Some(lhs)
    }
}
