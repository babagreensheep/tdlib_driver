use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::*;

/// combinator-decl ::= full-combinator-id { opt-args } { args } = result-type ;
#[derive(Debug, Clone, Lexer, Token)]
pub struct CombinatorDecl<'code> {
    /// - A combinator identifier is either an identifier starting with a lowercase Latin letter (lc-ident), or a namespace identifier (also lc-ident) followed by a period and another lc-ident. Therefore, cons and lists.get are valid combinator identifiers.
    /// - A combinator has a name, also known as a number (not to be confused with the designation) -- a 32-bit number that unambiguously determines it. It is either calculated automatically (see below) or it is explicitly assigned in the declaration. To do this, a hash mark (#) and exactly 8 hexadecimal digits -- the combinator's name -- are added to the identifier of the combinator being defined.
    object_name: FullCombinatorId<'code>,
    opt_args: Vec<OptArgs<'code>>,
    object_fields: Vec<Args<'code>>,
    _eq: PhantomData<Equals<'code>>,
    result: ResultType<'code>,
    _semi: PhantomData<Semicolon<'code>>,
}

impl<'code> Compile<Func> for CombinatorDecl<'code> {
    type State = Meta;

    async fn compile(self, meta: Self::State) -> Result<Func, String> {
        let Self {
            object_name,
            opt_args,
            object_fields,
            result,
            ..
        } = self;

        let object_name = object_name.compile(()).await?.to_string();

        let (result_type, generics) = result.compile(()).await?;

        // Transform the result to the base type if any
        let result_type_transformed = Type::transform_type(result_type.clone());

        // Create generics
        let generics = if generics.is_empty() {
            format!("")
        } else {
            let generics = generics.join(", ");
            format!("<{generics}>")
        };

        // Make a full result type
        let compiled_result_type = format!("{result_type_transformed}{generics}");

        // HACK: opt_args seems to apply to `vector` only, but throw a warning
        if !opt_args.is_empty() {
            println!("opt_args ({opt_args:?}) ignored as not used in current schema");
        }

        // Compile individual object values
        let object_fields = object_fields.compile(()).await?;

        // Push new object
        Function::push(Function {
            name: object_name,
            meta,
            fields: object_fields,
            result_type: compiled_result_type,
        });

        Ok(Func)
    }
}

impl<'code> Compile<Const> for CombinatorDecl<'code> {
    type State = Meta;

    async fn compile(self, meta: Self::State) -> Result<Const, String> {
        let Self {
            object_name,
            opt_args,
            object_fields,
            result,
            ..
        } = self;

        let object_name = object_name.compile(()).await?.to_string();

        let (result_type, _) = result.compile(()).await?;

        // Transform the result to the base type if any
        let result_type_transformed = Type::transform_type(result_type.clone());

        // HACK: opt_args seems to apply to `vector` only, but throw a warning
        if !opt_args.is_empty() {
            println!("opt_args ({opt_args:?}) ignored");
        }

        // Compile individual object values
        let object_fields = object_fields.compile(()).await?;

        // Create a new linkage of `object_name` -> `result_type`
        let object_name = Type::insert(
            object_name,
            Type::TD {
                meta: HashMap::new(),
                inherits_from: Some(result_type.clone()),
                children: Vec::new(),
                depends_on: object_fields
                    .iter()
                    .map(|object_value| object_value.type_id.clone())
                    .collect(),
            },
        )?;

        // Create linkage of `result_type.children` -> `object_name`
        // HACK: Hard coded to avoid `vector`
        if opt_args.is_empty() {
            match Type::write().entry(result_type_transformed.clone()) {
                Entry::Occupied(mut occupied_entry) => match occupied_entry.get_mut() {
                    Type::Rust(_) => (),
                    Type::TD { children, .. } => {
                        children.push((object_name.clone(), object_fields.len()))
                    }
                },
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(Type::TD {
                        meta: HashMap::new(),
                        inherits_from: None,
                        children: vec![(object_name.clone(), object_fields.len())],
                        depends_on: Vec::new(),
                    });
                }
            }
        }

        // Declare a new object only if there are fields on the object. IF there are no fields, the
        // object is not a true object and simply a Type.
        if !object_fields.is_empty() {
            // Push new object
            Object::push(Object {
                name: object_name,
                meta,
                fields: object_fields,
            })
        }

        Ok(Const)
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// full-combinator-id ::= lc-ident-full | _
pub enum FullCombinatorId<'code> {
    LcIdentFull(LcIdentFull<'code>),
    Underscore(Underscore<'code>),
}

impl<'code> Compile<LcIdent<'code>> for FullCombinatorId<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<LcIdent<'code>, String> {
        let _ = state;
        match self {
            FullCombinatorId::LcIdentFull(LcIdentFull(lc_ident_full, hex_code)) => {
                // Throw error for hex code
                if let Some(hex_code) = hex_code {
                    Err(format!("hex code ({hex_code:?}) not supported"))?;
                }
                let ident = lc_ident_full.compile(()).await?;
                Ok(ident)
            }
            FullCombinatorId::Underscore(_) => Err("anonymous identifier not supported".into()),
        }
    }
}

/// combinator-id ::= lc-ident-ns | _
#[derive(Debug, Clone, Lexer, Token)]
pub enum CombinatorId<'code> {
    LcIdentNs(LcIdentNs<'code>),
    Underscore(Underscore<'code>),
}

/// opt-args ::= { var-ident { var-ident } : [excl-mark] type-expr }
#[derive(Debug, Clone, Lexer, Token)]
pub struct OptArgs<'code> {
    _open: PhantomData<OpenBrace<'code>>,
    pub ident_0: VarIdent<'code>,
    pub ident_k: Vec<VarIdent<'code>>,
    _colon: PhantomData<Colon<'code>>,
    pub neg: Option<ExclMark<'code>>,
    pub expr: TypeExpr<'code>,
    _close: PhantomData<CloseBrace<'code>>,
}

/// args
#[derive(Debug, Clone, Lexer, Token)]
pub enum Args<'code> {
    /// args ::= var-ident-opt : [ conditional-def ] [ ! ] type-term
    _0 {
        field_name: VarIdentOpt<'code>,
        _colon: PhantomData<Colon<'code>>,
        cond: Option<ConditionalDef<'code>>,
        excl: Option<ExclMark<'code>>,
        field_type: TypeTerm<'code>,
    },
    /// args ::= [ var-ident-opt : ] [ multiplicity *] [ { args } ]
    _1(
        Option<Pair<'code, VarIdentOpt<'code>, Colon<'code>>>,
        Option<Pair<'code, Multiplicity<'code>, Asterisk<'code>>>,
        (OpenBracket<'code>, Vec<Args<'code>>, CloseBracket<'code>),
    ),
    /// args ::= ( var-ident-opt { var-ident-opt } : [!] type-term )
    _2(
        OpenPar<'code>,
        VarIdentOpt<'code>,
        Vec<VarIdentOpt<'code>>,
        Colon<'code>,
        Option<ExclMark<'code>>,
        TypeTerm<'code>,
        ClosePar<'code>,
    ),
    /// args ::= [ ! ] type-term
    _3(Option<ExclMark<'code>>, TypeTerm<'code>),
}

impl<'code> Compile<ObjectValue> for Args<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<ObjectValue, String> {
        let _ = state;
        let debug = format!("{self:?}");
        match self {
            Args::_0 {
                field_name,
                cond,
                excl,
                field_type,
                ..
            } => {
                let field_name = field_name.compile(()).await?;

                // Assertions
                if let Some(_) = cond {
                    Err(format!("condition (for @{field_name}) not in schema"))?;
                }

                // Assertions
                if let Some(_) = excl {
                    Err(format!("exclamation (for @{field_name}) not in schema"))?;
                }

                Ok(ObjectValue {
                    key: field_name,
                    type_id: field_type.compile(()).await?,
                })
            }
            Args::_1(..) => {
                println!("ignoring Args::_1: {debug}");
                Ok(ObjectValue {
                    key: "".to_string(),
                    type_id: "".to_string(),
                })
            }
            Args::_2(..) => Err(format!("Args::_2 not used: {debug}")),
            Args::_3(..) => {
                println!("ignoring Args::_3: {debug}");
                Ok(ObjectValue {
                    key: "".to_string(),
                    type_id: "".to_string(),
                })
            }
        }
    }
}

impl<'code> Compile<Vec<ObjectValue>> for Vec<Args<'code>> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<Vec<ObjectValue>, String> {
        let _ = state;
        let mut result = Vec::new();
        for arg in self.into_iter() {
            let arg = arg.compile(()).await?;

            // Reject empty objects (e.g. vector)
            if arg.key != "" && arg.type_id != "" {
                result.push(arg);
            }
        }
        Ok(result)
    }
}

/// multiplicity ::= nat-term
pub type Multiplicity<'code> = NatTerm<'code>;

#[derive(Debug, Clone, Lexer, Token)]
/// var-ident-opt ::= var-ident | _
pub enum VarIdentOpt<'code> {
    VarIdent(VarIdent<'code>),
    Underscore(Underscore<'code>),
}

impl<'code> Compile<String> for VarIdentOpt<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<String, String> {
        let _ = state;
        let mut ident = match self {
            VarIdentOpt::VarIdent(VarIdent::LcIdent(ident)) => Ok(ident.to_string()),
            VarIdentOpt::VarIdent(VarIdent::UcIdent(ident)) => Ok(ident.to_string()),
            VarIdentOpt::Underscore(_) => Err("anonymous identifier not supported".to_string()),
        }?;
        if ident == "type" {
            ident = "r#type".to_string();
        }
        Ok(ident)
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// conditional-def ::= var-ident [ . nat-const ] ?
pub struct ConditionalDef<'code>(
    VarIdent<'code>,
    Option<PuncNatConst<'code>>,
    QuestionMark<'code>,
);

#[derive(Debug, Clone, Lexer, Token)]
/// punc_nat_const ::= . nat-const
pub struct PuncNatConst<'code>(PhantomData<Dot<'code>>, NatConst<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// result-type
pub enum ResultType<'code> {
    /// result-type ::= boxed-type-ident { subexpr }
    _00(BoxedTypeIdent<'code>, Vec<Subexpr<'code>>),
    /// result-type ::= boxed-type-ident < subexpr { , subexpr } >
    _02(
        BoxedTypeIdent<'code>,
        Langle<'code>,
        Subexpr<'code>,
        Vec<Pair<'code, Comma<'code>, Subexpr<'code>>>,
        Rangle<'code>,
    ),
}

impl<'code> Compile<(String, Vec<String>)> for ResultType<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<(String, Vec<String>), String> {
        let _ = state;
        let debug = format!("{self:?}");
        match self {
            ResultType::_00(uc_ident_ns, vec) => {
                let ident = uc_ident_ns.compile(()).await?;

                let generics_future = vec.into_iter().map(|sub_expr| match sub_expr {
                    Subexpr::Term(Term::TypeIdent(TypeIdent::LcIdentNs(ident))) => Ok(ident),
                    _ => Err(format!("expression not used in schema: {sub_expr:?}")),
                });

                let mut generics = Vec::new();
                for generic in generics_future {
                    let generic = generic?;
                    let generic = generic.compile(()).await?;
                    generics.push(generic.to_string());
                }

                Ok((ident.to_string(), generics))
            }
            ResultType::_02(..) => Err(format!("result-type::_2 not used in schema: {debug}")),
        }
    }
}

// impl<'code> Compile<> for ResultType<'code> {}

#[derive(Debug, Clone, Lexer, Token)]
/// builtin-combinator-decl ::= full-combinator-id ? = boxed-type-ident ;
pub struct BuiltinCombinatorDecl<'code> {
    pub type_name: FullCombinatorId<'code>,
    _qm: PhantomData<QuestionMark<'code>>,
    _eq: PhantomData<Equals<'code>>,
    pub result_type: BoxedTypeIdent<'code>,
    _semi: Semicolon<'code>,
}

impl<'code> Compile<()> for BuiltinCombinatorDecl<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<(), String> {
        let _ = state;
        let BuiltinCombinatorDecl {
            type_name,
            result_type,
            ..
        } = self;

        let UcIdentNs {
            namespace,
            type_name: result_type,
        } = result_type;

        // Namespaced should be blank
        if let Some(Pair(ns, _, _, _)) = namespace {
            Err(format!("namespace ({ns}) not supported"))?
        }

        // Obtain the matching rust type
        let rust_type = Type::from_schema_type(&result_type).ok_or("{ident} is not a base type")?;

        // Compile type_name as a string
        let type_name = type_name.compile(()).await?.to_string();

        // Insert the new rust type
        Type::insert(type_name, rust_type)?;

        Ok(())
    }
}

//////////////////////////////////////
//////////////////////////////////////
///              PARSE             ///
///              TESTS             ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
mod parse_tests;
