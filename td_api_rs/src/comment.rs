use super::*;

#[derive(Debug, Clone, Lexer, Token)]
struct CommentMark<'code>(
    #[pattern("//")]
    Buffer<'code>,
    Spaces<'code>,
);

#[derive(Debug, Clone, Lexer, Token)]
struct FieldMark<'code>(#[pattern("@")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
pub struct ClassDesc<'code> {
    /// Opening
    _mark: CommentMark<'code>,
    /// Optional spaces
    _mark_space: Spaces<'code>,
    /// @ symbol
    _field_mark: FieldMark<'code>,
    /// Ident
    #[pattern("class")]
    _field: Buffer<'code>,
    /// Space,
    _field_space: Spaces<'code>,
    /// Ident
    type_ident: TypeIdent<'code>,
    /// Space,
    _type_space: Spaces<'code>,
    /// meta
    meta: Vec<Desc<'code>>,
}

impl<'code> Compile<()> for ClassDesc<'code> {
    type State = ();
    async fn compile(self, _state: Self::State) -> Result<(), String> {
        let debug_buffer = format!("{:?}", self.buffer());
        let Self {
            _mark,
            _mark_space,
            _field_mark,
            _field,
            _field_space,
            type_ident,
            _type_space,
            meta,
        } = self;

        let type_ident = type_ident
            .buffer()
            .ok_or(format!("missing name: {debug_buffer}"))?
            .to_string();

        // Compile metadata
        let meta = meta
            .into_iter()
            .map(|desc| {
                let field = desc
                    .field
                    .buffer()
                    .map(|b| b.to_string())
                    .unwrap_or("unknown".into());
                let desc_text = desc.to_string();

                (field, desc_text)
            })
            .collect();

        // Insert metadata
        match Type::write().entry(type_ident) {
            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                if let Type::TD { meta: old_meta, .. } = occupied_entry.get_mut() {
                    *old_meta = meta;
                }
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(Type::TD {
                    meta,
                    inherits_from: None,
                    children: Vec::new(),
                    depends_on: Vec::new(),
                });
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// Type of document comment
/// desc-comment ::= // desc | desc
pub enum DescComment<'code> {
    NonCommented(Desc<'code>),
    Commented {
        /// Opening
        #[pattern("//")]
        _mark: Buffer<'code>,
        /// Optional spaces
        _mark_space: Spaces<'code>,
        /// Desc
        desc: Desc<'code>,
    },
}

impl<'code> Compile<Meta> for Vec<DescComment<'code>> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<Meta, String> {
        let meta = self
            .into_iter()
            .map(|comment_block| match comment_block {
                DescComment::NonCommented(desc) => desc,
                DescComment::Commented {
                    _mark,
                    _mark_space,
                    desc,
                } => desc,
            })
            .collect::<Vec<_>>();
        meta.compile(state).await
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// desc ::= @ var-ident {*} { extended-desc }
pub struct Desc<'code> {
    /// @ symbol
    _field_mark: FieldMark<'code>,
    /// Ident
    pub field: VarIdent<'code>,
    /// Space,
    _field_space: Spaces<'code>,
    /// Any
    pub text: Group<'code, CommentChar<'code>, 0>,
    /// Spaces
    _text_space: Spaces<'code>,
    /// Extended comments
    pub text_extended: Vec<ExtendedDesc<'code>>,
}

impl<'code> Compile<Meta> for Vec<Desc<'code>> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<Meta, String> {
        let _ = state;
        Ok(self
            .into_iter()
            .map(|desc| {
                let field = desc
                    .field
                    .buffer()
                    .map(|b| b.to_string())
                    .unwrap_or("".into());
                let desc_text = desc.to_string();

                (field, desc_text)
            })
            .collect())
    }
}

impl<'code> std::fmt::Display for Desc<'code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description = self
            .text
            .buffer()
            .map(|b| b.to_string())
            .unwrap_or("".into());

        description.push_str(
            &self
                .text_extended
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        );

        description.fmt(f)
    }
}

#[derive(Debug, Clone, Lexer, Token)]
pub struct ExtendedDesc<'code> {
    /// Opening
    #[pattern("//")]
    _mark: Buffer<'code>,
    /// Optional spaces
    _mark_space: Spaces<'code>,
    /// Not field marker
    _not_field_mark: Not<FieldMark<'code>>,
    /// Any
    text: Group<'code, CommentChar<'code>, 0>,
    /// Spaces
    _text_space: Spaces<'code>,
}

impl<'code> std::fmt::Display for ExtendedDesc<'code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = self
            .buffer()
            .map(|buffer| buffer.to_string())
            .unwrap_or("".to_string());
        buffer.fmt(f)
    }
}

#[derive(Debug, Clone, Lexer, Token)]
pub struct CommentChar<'code>(
    Not<NewLine<'code>>,
    Not<FieldMark<'code>>,
    Not<CommentMark<'code>>,
    Buffer<'code>,
);

//////////////////////////////////////
//////////////////////////////////////
///             COMPILE            ///
///              TESTS             ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
mod compile_tests;

//////////////////////////////////////
//////////////////////////////////////
///              PARSE             ///
///              TESTS             ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
mod parse_tests;
