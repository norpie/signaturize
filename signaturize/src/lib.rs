use std::fmt::Display;

use serde::{Deserialize, Serialize};
pub use signaturize_derive::Signature;

extern crate signaturize_derive;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Signature {
    Type(String),
    Field {
        name: Box<Signature>,
        value: Box<Signature>,
    },
    Struct {
        name: Box<Signature>,
        fields: Vec<Signature>,
    },
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signature::Type(value) => {
                write!(f, "{}", value)
            }
            Signature::Field { name, value } => {
                write!(f, "{}: {}", name, value)
            }
            Signature::Struct { name, fields } => {
                let lines = Self::fmt_struct(name, fields, 0);
                write!(f, "{}", lines.join("\n"))
            }
        }
    }
}

impl Signature {
    pub fn fmt_struct(name: &Signature, fields: &Vec<Signature>, amount: usize) -> Vec<String> {
        let indent = " ".repeat(amount * 4);
        let next_indent = " ".repeat((amount + 1) * 4);
        let mut output = Vec::new();
        output.push(format!("{}{} {{", indent, name));
        for field in fields {
            match field {
                Signature::Field { name, value } => match &**value {
                    Signature::Struct { name, fields } => {
                        let lines = Self::fmt_struct(name, fields, amount + 1);
                        output.extend(lines);
                    }
                    _ => {
                        output.push(format!("{}{}: {}", next_indent, name, value));
                    }
                },
                _ => {
                    panic!("Illegal signature");
                }
            }
        }
        output.push(format!("{}}}", indent));
        output
    }
}

pub trait Signaturize {
    fn signature() -> Signature;
}

macro_rules! signature_of {
    ($ty:ty) => {
        impl Signaturize for $ty {
            fn signature() -> Signature {
                Signature::Type(stringify!($ty).to_string())
            }
        }
    };
}

// Vectors
impl<T> Signaturize for Vec<T>
where
    T: Signaturize,
{
    fn signature() -> Signature {
        Signature::Type(format!("Vec<{}>", T::signature()))
    }
}

// Strings
impl Signaturize for str {
    fn signature() -> Signature {
        Signature::Type("String".to_string())
    }
}
signature_of!(String);

// Primitives
signature_of!(bool);
signature_of!(char);

// Floats
signature_of!(f32);
signature_of!(f64);
// signature_of!(f128);

// Ints
signature_of!(isize);
signature_of!(i8);
signature_of!(i16);
signature_of!(i32);
signature_of!(i64);
signature_of!(i128);

// Unsigned Ints
signature_of!(usize);
signature_of!(u8);
signature_of!(u16);
signature_of!(u32);
signature_of!(u64);
signature_of!(u128);
