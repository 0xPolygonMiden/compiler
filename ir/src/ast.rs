//! The AST definition of Miden IR.
//!

pub enum Literal {
    BoolLit(bool),
    U32Lit(u32),
}

pub enum IntTyp {
    U32,
}

pub enum Type {
    BoolType,
    IntType(IntTyp)
}

pub struct Variable {
    name : String,
    typ : Type
}

/// Behavior in case of over- or underflow
pub enum OfBehavior {
    Wrapping,
    Checked,
}
    
/// Integer operations wrap in case of over- and underflows.
pub enum IntBinop {
    Add(IntTyp, OfBehavior),
    Sub(IntTyp, OfBehavior),
    Mul(IntTyp, OfBehavior),
    Div(IntTyp),
    Rem(IntTyp),
    //BitOr(IntTyp),
    //BitAnd(IntTyp),
    //BitXor(IntTyp),
}

pub enum BoolBinop {
    Lor,
    Land,
}

pub enum BoolUnop {
    Lneg
}

pub enum Cmp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

pub enum AtomicExpression<'a> {
    LitExp(Literal),
    VarExp(&'a Variable),
}

pub enum Expression<'a> {
    AtomicExp(AtomicExpression<'a>),
    IntBinopExp(IntBinop, AtomicExpression<'a>, AtomicExpression<'a>),
    //CastExp(IntTyp, Expression),
    BoolUnopExp(BoolUnop, AtomicExpression<'a>),
    BoolBinopExp(BoolBinop, AtomicExpression<'a>, AtomicExpression<'a>),
    CmpExp(Cmp, AtomicExpression<'a>, AtomicExpression<'a>),
    FunctionCallExp(&'a Function<'a>, Vec<AtomicExpression<'a>>), // only calls to previously defined functions allowed
}

pub enum StatementExpression<'a> {
    VarDecl(Variable, Expression<'a>),
}

pub enum Statement<'a> {
    ExpressionStmt(StatementExpression<'a>),
    ReturnStmt(AtomicExpression<'a>),
    IfThenElse(AtomicExpression<'a>, Vec<Statement<'a>>, Vec<Statement<'a>>),
    WhileStmt(Vec<StatementExpression<'a>>, AtomicExpression<'a>, Vec<Statement<'a>>),
}

pub struct Function<'a> {
    name: String,
    params: Vec<Variable>,
    body: Vec<Statement<'a>>,
    return_typ : Option<Type>,
}

pub struct Program<'a> {
    functions: Vec<Function<'a>>,
    main_function: Function<'a>,
}
