use super::*;

/// partial-app-decl ::= partial-type-app-decl | partial-comb-app-decl
#[derive(Debug, Clone, Lexer, Token)]
pub enum PartialAppDecl<'code> {
    PartialTypeAppDecl(PartialTypeAppDecl<'code>),
    PartialCombAppDecl(PartialCombAppDecl<'code>),
}
/// partial-type-app-decl ::= boxed-type-ident subexpr { subexpr } ; | boxed-type-ident < expr { , expr } > ;
#[derive(Debug, Clone, Lexer, Token)]
pub enum PartialTypeAppDecl<'code> {
    BoxedSubexpr(
        BoxedTypeIdent<'code>,
        Subexpr<'code>,
        Vec<Subexpr<'code>>,
        Semicolon<'code>,
    ),
    Boxed(
        BoxedTypeIdent<'code>,
        Langle<'code>,
        Expr<'code>,
        Vec<Pair<'code, Comma<'code>, Expr<'code>>>,
        Rangle<'code>,
        Semicolon<'code>,
    ),
}
/// partial-comb-app-decl ::= combinator-id subexpr { subexpr } ;
#[derive(Debug, Clone, Lexer, Token)]
pub struct PartialCombAppDecl<'code>(
    CombinatorId<'code>,
    Subexpr<'code>,
    Vec<Subexpr<'code>>,
    Semicolon<'code>,
);
