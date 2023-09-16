use super::super::types::Type;
use super::Expression;
use crate::lexer::*;
use std::io::Read;

impl Expression {
    // fn binary_expression<R: Read>(
    //     tokens: &mut TokenBuffer<R>,
    //     precedance: u32,
    // ) -> Result<Option<Self>> {
    //     let lhs = match (precedance as i32) < 0 {
    //         true => Self::binary_expression(tokens, precedance + 1)?,
    //         false => Self::literal(tokens)?,
    //     };
    //     let lhs = match lhs {
    //         Some(lhs) => lhs,
    //         None => return Ok(None),
    //     };

    //     match precedance {
    //         0 => {
    //             if tokens.match_keyword(Keyword::As)? {
    //                 return Ok(Some(Expression::TypeCast(
    //                     Box::new(lhs),
    //                     super::types::Type::parse(tokens)?,
    //                 )));
    //             }
    //         }
    //         precedance => panic!(
    //             "Internal error: invalid binary operator precedance: {}",
    //             precedance
    //         ),
    //     }
    //     Ok(Some(lhs))
    // }

    // pub(super) fn parse_lazy_boolean<R: Read>(tokens: &mut TokenBuffer<R>) -> Result<Option<Self>> {
    //     let mut lhs = match Self::parse_typecast(tokens)? {
    //         Some(lhs) => lhs,
    //         None => return Ok(None),
    //     };

    //     // while tokens.match_keyword(Keyword::As)? {
    //     //     lhs = Expression::TypeCast(Box::new(lhs), Type::parse(tokens)?);
    //     // }
    //     Ok(Some(lhs))
    // }

    pub(super) fn parse_typecast<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Self> {
        let mut lhs = Self::parse_literal(tokens)?;

        while tokens.match_keyword(Keyword::As) {
            lhs = Expression::TypeCast(Box::new(lhs), Type::parse(tokens));
        }
        Some(lhs)
    }
}
