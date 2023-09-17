use super::*;
use orecc_back::BinaryOperation;
use std::io::Read;

macro_rules! binary_expression {
    ($name: ident, $prev: path: $($operator: ident -> $operation: ident;)+) => {
        pub fn $name<R: Read, B: Backend>(
            tokens: &mut TokenBuffer<R>,
            backend: &mut B,
        ) -> Option<B::DataType> {
            let mut lhs = $prev(tokens, backend)?;
            loop {
                $(if tokens.match_operator(Operator::$operator) {
                    if let Some(rhs) = $prev(tokens, backend) {
                        lhs = backend.binary_operation(BinaryOperation::$operation, lhs, rhs);
                        continue;
                    } else {
                        tokens.error("expected expression");
                    }
                })+
                break;
            }
            Some(lhs)
        }
    };
}

binary_expression! {
    parse_logical_or, parse_logical_and:
    LogicalOr -> LogicalOr;
}

binary_expression! {
    parse_logical_and, parse_bitwise_or:
    LogicalAnd -> LogicalAnd;
}

binary_expression! {
    parse_bitwise_or, parse_bitwise_xor:
    Bar -> BitwiseOr;
}

binary_expression! {
    parse_bitwise_xor, parse_bitwise_and:
    Carrot -> BitwiseXor;
}

binary_expression! {
    parse_bitwise_and, parse_shift:
    Ampersand -> BitwiseAnd;
}

binary_expression! {
    parse_shift, parse_aditive:
    ShiftLeft -> ShiftLeft;
    ShiftRight -> ShiftRight;
}

binary_expression! {
    parse_aditive, parse_multiplicative:
    Plus -> Add;
    Minus -> Subtract;
}

binary_expression! {
    parse_multiplicative, parse_typecast:
    Star -> Multiply;
    Slash -> Divide;
    Modulo -> DivisionReminder;
}

pub fn parse_typecast<R: Read, B: Backend>(
    tokens: &mut TokenBuffer<R>,
    backend: &mut B,
) -> Option<B::DataType> {
    let mut lhs = super::parse_literal(tokens, backend)?;

    // while tokens.match_keyword(Keyword::As) {
    //     lhs = Expression::TypeCast(Box::new(lhs), Type::parse(tokens));
    // }
    Some(lhs)
}
