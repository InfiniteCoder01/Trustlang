use crate::lexer::*;
use std::io::Read;

#[derive(Clone, Debug, Default)]
pub enum Type {
    #[default]
    Unknown,
    Bool,
    Sized(SizedType, TypeSize),
}

#[derive(Clone, Copy, Debug)]
pub enum SizedType {
    Integer,
    Unsigned,
    Float,
}

#[derive(Clone, Copy, Debug)]
pub enum TypeSize {
    Bits(u16),
    SizeT,
}

impl Type {
    pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Self {
        if let Some(token) = tokens.next_token() {
            let ty = if let Some(ty) = Self::parse_sized(&token) {
                ty
            } else {
                match token {
                    Token::Ident(_, Some(Keyword::Bool)) => Type::Bool,
                    token => {
                        tokens.error(format!("expected type, got {token}"));
                        Self::Unknown
                    }
                }
            };
            ty
        } else {
            tokens.error("expected type");
            Self::Unknown
        }
    }

    fn parse_sized(token: &Token) -> Option<Type> {
        if let Token::Ident(literal, _) = &token {
            let (ty, size) = if let Some(size) = literal.strip_prefix('i') {
                (SizedType::Integer, size)
            } else if let Some(size) = literal.strip_prefix('u') {
                (SizedType::Unsigned, size)
            } else if let Some(size) = literal.strip_prefix('f') {
                (SizedType::Float, size)
            } else {
                return None;
            };

            if let Some(size) = TypeSize::parse(ty, size) {
                return Some(Type::Sized(ty, size));
            }
        }
        None
    }
}

impl TypeSize {
    fn parse(ty: SizedType, size: &str) -> Option<TypeSize> {
        Some(if size == "size" {
            match ty {
                SizedType::Integer | SizedType::Unsigned => TypeSize::SizeT,
                SizedType::Float => return None,
            }
        } else {
            let size = size.parse::<u16>().ok()?;
            if match ty {
                SizedType::Integer | SizedType::Unsigned => &[8, 16, 32, 64, 128][..],
                SizedType::Float => &[32, 64][..],
            }
            .binary_search(&size)
            .is_err()
            {
                return None;
            }
            TypeSize::Bits(size)
        })
    }
}
