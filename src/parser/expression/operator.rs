use super::*;
use std::io::Read;

pub fn binary<R: Read>(tokens: &mut TokenBuffer<R>, precedance: usize) -> Option<Expression> {
    let transitions = [
        &[(Operator::LogicalOr, BinaryOperation::LogicalOr)] as &[(Operator, BinaryOperation)],
        &[(Operator::LogicalAnd, BinaryOperation::LogicalAnd)],
        &[(Operator::Bar, BinaryOperation::BitwiseOr)],
        &[(Operator::Carrot, BinaryOperation::BitwiseXor)],
        &[(Operator::Ampersand, BinaryOperation::BitwiseAnd)],
        &[
            (Operator::ShiftLeft, BinaryOperation::ShiftLeft),
            (Operator::ShiftRight, BinaryOperation::ShiftRight),
        ],
        &[
            (Operator::Plus, BinaryOperation::Add),
            (Operator::Minus, BinaryOperation::Subtract),
        ],
        &[
            (Operator::Star, BinaryOperation::Multiply),
            (Operator::Slash, BinaryOperation::Divide),
            (Operator::Modulo, BinaryOperation::DivisionReminder),
        ],
    ];

    if precedance >= transitions.len() {
        return super::literal(tokens);
    }

    let mut lhs = binary(tokens, precedance + 1)?;
    'chain: loop {
        for &(operator, operation) in transitions[precedance] {
            if tokens.match_operator(operator) {
                if let Some(rhs) = binary(tokens, precedance + 1) {
                    lhs = Expression::Binary(Box::new(lhs), operation, Box::new(rhs));
                    continue 'chain;
                } else {
                    let got = tokens.got_token();
                    tokens.error(format!("expected expression, got {}", got));
                    break 'chain;
                }
            }
        }
        break;
    }
    Some(lhs)
}

// pub fn parse_typecast<R: Read, B: Backend>(
//     tokens: &mut TokenBuffer<R>,
//     backend: &mut B,
// ) -> Option<ExpressionNode<B::DataType>> {
//     let mut lhs = super::parse_literal(tokens, backend)?;
//
//     while tokens.match_keyword(Keyword::As) {
//         lhs = Expression::TypeCast(Box::new(lhs), Type::parse(tokens));
//     }
//     Some(lhs)
// }
