#[derive(Debug, PartialEq)]
pub struct Script<'s> {
    pub statements: Box<[Statement<'s>]>
}

#[derive(Debug, PartialEq)]
pub enum Statement<'s> {
    VariableDeclaration(VariableDeclaration<'s>),
}

#[derive(Debug, PartialEq)]
pub enum DeclarationKind {
    Var
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration<'s> {
    pub kind: DeclarationKind,
    pub declarators: Box<[VariableDeclarator<'s>]>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarator<'s> {
    pub binding: Binding<'s>,
    pub init: Option<Expression<'s>>,
}

#[derive(Debug, PartialEq)]
pub enum Binding<'s> {
    Identifier(Identifier<'s>),
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'s> {
    pub name: &'s str
}

#[derive(Debug, PartialEq)]
pub enum Expression<'s> {
    LiteralString(LiteralString<'s>)
}

#[derive(Debug, PartialEq)]
pub struct LiteralString<'s> {
    pub value: &'s str
}
