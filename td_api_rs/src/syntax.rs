#![allow(dead_code)]
use super::*;

/// Expression for a Type in TDLib
/// `type-expr ::= expr`
pub type TypeExpr<'code> = Expr<'code>;

/// Expression for a numeric constant in TDLib
/// `nat-expr ::= expr`
pub type NatExpr<'code> = Expr<'code>;

/// expr ::= { subexpr }
pub type Expr<'code> = Vec<Subexpr<'code>>;

impl<'code> Compile<String> for Expr<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<String, String> {
        let _ = state;
        if self.len() > 1 {
            unreachable!("chained expressions cannot be compiled into a string")
        }

        let expr = self
            .into_iter()
            .next()
            .ok_or(format!("no expressions found"))?;

        Box::pin(expr.compile(())).await
    }
}

/// subexpr ::= term | nat-const + subexpr | subexpr + nat-const
#[derive(Debug, Clone, Lexer, Token)]
pub enum Subexpr<'code> {
    /// Denotes a term
    Term(Term<'code>),

    /// NOTE: Not used in schema.
    NatConstSubexpr(NatConst<'code>, Plus<'code>, Box<Subexpr<'code>>),

    /// NOTE: following the BNF in the schema documents results in a recursive parsing. As such,
    /// this has been optmimised into the Term + NatConst instead
    /// `subexpr_nat_const(Box<subexpr<'code>>, plus<'code>, nat_const<'code>),`
    SubexprNatConst(Term<'code>, Plus<'code>, NatConst<'code>),
}

impl<'code> Compile<String> for Subexpr<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<String, String> {
        let debug = format!("{self:?}");
        match self {
            Subexpr::Term(term) => term.compile(state).await,
            Subexpr::NatConstSubexpr(..) => Err(format!(
                "Subexpr::NatConstSubexpr cannot convert to string in current schema: {debug}"
            )),
            Subexpr::SubexprNatConst(..) => Err(format!(
                "Subexpr::SubexprNatConst cannot convert to string in current schema: {debug}"
            )),
        }
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// term ::= ( expr ) | type-ident | var-ident | nat-const | % term | type-ident < expr { , expr } >
pub enum Term<'code> {
    /// NOTE: Not currently used in schema
    Expr(OpenPar<'code>, Expr<'code>, ClosePar<'code>),

    /// Identifier for a Type with generics
    /// NOTE: This is moved up from the specified order in the schema to ensure that it gets parsed
    /// first. The Lexer would otherwise identify [Term::TypeIdent] and never parse the generics.
    TypeIdentExpanded {
        type_name: TypeIdent<'code>,
        _l: PhantomData<Langle<'code>>,
        first_generic: Expr<'code>,
        subsequent_generics: Vec<Pair<'code, Comma<'code>, Expr<'code>>>,
        _r: PhantomData<Rangle<'code>>,
    },

    /// Identifier for a Type
    TypeIdent(TypeIdent<'code>),

    /// Identifier for a Variable
    VarIdent(VarIdent<'code>),

    /// Identifier for a numeric constant
    NatConst(NatConst<'code>),

    /// ???
    PercentTerm(Percent<'code>, Box<Term<'code>>),
}

impl<'code> Compile<String> for Term<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<String, String> {
        let _ = state;
        let debug = format!("{self:?}");
        match self {
            Term::TypeIdentExpanded {
                type_name,
                first_generic,
                subsequent_generics,
                ..
            } => {
                let type_name = type_name.compile(()).await?;

                // Transform to the base type, if any
                let type_name = Type::transform_type(type_name);

                // Create generics
                let first_generic = first_generic.compile(()).await?;
                let mut generics = vec![first_generic];
                for Pair(_, generic, _, _) in subsequent_generics {
                    let generic = generic.compile(()).await?;
                    let generic = Type::transform_type(generic);
                    generics.push(generic);
                }

                // Generate generics string
                let generics = generics.join(", ");

                // Combine generics into typename and return
                // HACK: It is correct to assume that there are generics since there are additional
                // statments
                Ok(format!("{type_name}<{generics}>"))
            }
            Term::TypeIdent(type_name) => {
                let type_name = type_name.compile(()).await?;
                Ok(Type::transform_type(type_name))
            }
            Term::Expr(..) => Err(format!("Term::Expr not used in schema: {debug}")),
            Term::VarIdent(var_ident) => var_ident.compile(()).await,
            Term::NatConst(..) => Err(format!("Term::NatConst not used in schema: {debug}")),
            Term::PercentTerm(..) => Err(format!("Term::PercentTerm not used in schema: {debug}")),
        }
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// type-ident ::= boxed-type-ident | lc-ident-ns | #
pub enum TypeIdent<'code> {
    BoxedTypeIdent(BoxedTypeIdent<'code>),
    LcIdentNs(LcIdentNs<'code>),
    Hash(Hash<'code>),
}

impl<'code> Compile<String> for TypeIdent<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<String, String> {
        let _ = state;
        let debug = format!("{self:?}");
        match self {
            TypeIdent::BoxedTypeIdent(uc_ident_ns) => {
                let uc_ident_ns = uc_ident_ns.compile(()).await?;
                Ok(uc_ident_ns.to_string())
            }
            TypeIdent::LcIdentNs(lc_ident_ns) => {
                let lc_ident_ns = lc_ident_ns.compile(()).await?;
                Ok(lc_ident_ns.to_string())
            }
            TypeIdent::Hash(..) => Err(format!("TypeIdent::Hash not used in schema: {debug}")),
        }
    }
}

/// boxed-type-ident ::= uc-ident-ns
pub type BoxedTypeIdent<'code> = UcIdentNs<'code>;

#[derive(Debug, Clone, Lexer, Token)]
/// var-ident ::= lc-ident | uc-ident
pub enum VarIdent<'code> {
    LcIdent(LcIdent<'code>),
    UcIdent(UcIdent<'code>),
}

impl<'code> Compile<String> for VarIdent<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<String, String> {
        let _ = state;
        match self {
            VarIdent::LcIdent(ident) => Ok(ident.to_string()),
            VarIdent::UcIdent(ident) => Ok(ident.to_string()),
        }
    }
}

/// type-term ::= term
pub type TypeTerm<'code> = Term<'code>;

/// nat-term ::= term
pub type NatTerm<'code> = Term<'code>;

//////////////////////////////////////
//////////////////////////////////////
///              PARSE             ///
///              TESTS             ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
mod parse_tests;
