#[derive(Debug)]
pub struct AST<'a> {
    pub statements: Vec<Span<'a, TLStatement<'a>>>,
}

#[derive(Debug)]
pub struct Span<'a, T> {
    span: &'a str,
    node: T,
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: Span<'a, String>,
    pub args: Vec<Span<'a, String>>,
    pub body: Vec<Span<'a, Statement<'a>>>,
}

#[derive(Debug)]
pub enum TLStatement<'a> {
    Function(Span<'a, Function<'a>>),
}

#[derive(Debug)]
pub enum Statement<'a> {
    Expression(Span<'a, Expression<'a>>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    Atom(Span<'a, Atom>),
    BinaryOp {
        left: Box<Span<'a, Expression<'a>>>,
        operator: Operator,
        right: Box<Span<'a, Expression<'a>>>,
    },
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum Atom {
    Num(i32),
}
