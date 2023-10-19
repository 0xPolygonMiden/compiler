use super::*;
use crate::{Ident, Type};

const INDENT: &str = "    ";

/// Represents the label at the start of a basic block.
///
/// Labels must be unique within each function.
#[derive(PartialEq, Debug)]
pub struct Label {
    pub name: Ident,
}
impl Label {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }
}
impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Represents an argument for a basic block
#[derive(PartialEq, Debug)]
pub struct BlockArgument {
    pub value: Value,
    pub ty: Type,
}
impl BlockArgument {
    pub fn new(value: Value, ty: Type) -> Self {
        Self { value, ty }
    }
}
impl fmt::Display for BlockArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {} ", self.value, self.ty)
    }
}

/// Represents the label and the arguments of a basic block
#[derive(PartialEq, Debug)]
pub struct BlockHeader {
    pub label: Label,
    pub args: Vec<BlockArgument>,
}
impl BlockHeader {
    pub fn new(label: Label, args: Vec<BlockArgument>) -> Self {
        Self { label, args }
    }
}
impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.label)?;
        if self.args.is_empty() {
            f.write_str(":\n")
        } else {
            f.write_str("(")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i != 0 {
                    write!(f, ", {}", arg)?;
                } else {
                    write!(f, "{}", arg)?;
                }
            }
            f.write_str(") :\n")
        }
    }
}

/// Represents a basic block of instructions
#[derive(Spanned, Debug)]
pub struct Block {
    #[span]
    pub span: SourceSpan,
    pub header: BlockHeader,
    pub instructions: Vec<Instruction>,
}
impl Block {
    pub fn new(span: SourceSpan, header: BlockHeader, instructions: Vec<Instruction>) -> Self {
        Self {
            span,
            header,
            instructions,
        }
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.instructions == other.instructions
    }
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for inst in self.instructions.iter() {
            writeln!(f, "{}{}", INDENT, inst)?;
        }
        Ok(())
    }
}
