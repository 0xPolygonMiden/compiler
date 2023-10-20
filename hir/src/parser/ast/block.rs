use crate::{Ident, Type, };
use super::*;

const INDENT: &str = "    ";

/// Represents the label at the start of a basic block.
///
/// Labels must be unique within each function.
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
        if self.args.len() == 0 {
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
#[derive(Spanned)]
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
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for inst in self.instructions.iter() {
            writeln!(f, "{}{}", INDENT, inst)?;
        }
        Ok(())
    }
}
