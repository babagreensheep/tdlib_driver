//! TODO: Optimise for loops

// #![allow(unused_variables, dead_code)]

use lexerus::*;
use std::io::Write;
use std::marker::PhantomData;

#[derive(Debug, Clone, Lexer, Token)]
struct Pair<'code, First: Lexer<'code>, Second: Lexer<'code>>(
    First,
    Second,
    #[build_with(Ok(PhantomData))] PhantomData<&'code First>,
    #[build_with(Ok(PhantomData))] PhantomData<&'code Second>,
);

type Spaces<'code> = PhantomData<Group<'code, WhiteSpace<'code>, 0>>;

mod compile;
use compile::*;

mod base;
use base::*;

mod syntax;
use syntax::*;

mod combinator;
use combinator::*;

mod partial;
use partial::*;

mod comment;
use comment::*;

/// final-decl ::= New boxed-type-ident ; | Final boxed-type-ident ; | Empty boxed-type-ident ;
#[derive(Debug, Clone, Lexer, Token)]
enum FinalDecl<'code> {
    NewKw(NewKw<'code>, BoxedTypeIdent<'code>, Semicolon<'code>),
    FinalKw(FinalKw<'code>, BoxedTypeIdent<'code>, Semicolon<'code>),
    EmptyKw(EmptyKw<'code>, BoxedTypeIdent<'code>, Semicolon<'code>),
}

#[derive(Debug, Clone, Lexer, Token)]
/// TL-program ::= constr-declarations { --- functions --- fun-declarations | --- types --- constr-declarations }
struct TlProgram<'code>(ConstrDeclarations<'code>, Vec<Declarations<'code>>, EoF);

impl<'code> Compile<()> for TlProgram<'code> {
    type State = ();

    async fn compile(self, _state: Self::State) -> Result<(), String> {
        let Self(const_strs, mixed_decl, ..) = self;

        // Type declarations
        compile_constr_declaration_block(const_strs).await?;

        // Compile the vector of bullshit
        for declaration in mixed_decl {
            declaration.compile(()).await?;
            // tokio::task::spawn(declaration.compile(())).await.unwrap()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Lexer, Token)]
enum Declarations<'code> {
    Functions(
        TripleMinus<'code>,
        #[pattern("functions")] Buffer<'code>,
        TripleMinus<'code>,
        FunDeclarations<'code>,
    ),
    Types(
        TripleMinus<'code>,
        #[pattern("types")] Buffer<'code>,
        TripleMinus<'code>,
        ConstrDeclarations<'code>,
    ),
}

impl<'code> Compile<()> for Declarations<'code> {
    type State = ();

    async fn compile(self, _state: Self::State) -> Result<(), String> {
        match self {
            Declarations::Functions(_, _, _, declarations) => {
                compile_func_declaration_block(declarations).await
            }
            Declarations::Types(_, _, _, declarations) => {
                compile_constr_declaration_block(declarations).await
            }
        }
    }
}

/// constr-declarations ::= { declaration }
type ConstrDeclarations<'code> = Vec<Block<'code>>;

/// fun-declarations ::= { declaration }
type FunDeclarations<'code> = Vec<Block<'code>>;

#[derive(Debug, Clone, Lexer, Token)]
/// declaration ::= combinator-decl | partial-app-decl | final-decl
enum Declaration<'code> {
    Combinator(CombinatorDecl<'code>),
    BuiltinCombinator(BuiltinCombinatorDecl<'code>),
    PartialAppDecl(PartialAppDecl<'code>),
    FinalDecl(FinalDecl<'code>),
}

#[derive(Debug, Clone, Lexer, Token)]
/// block ::= enum_decl | desc_decl
enum Block<'code> {
    EnumDecl(ClassDesc<'code>),
    DescDecl(DeclBlock<'code>),
}

#[derive(Debug, Clone, Lexer, Token)]
/// desc_decl ::= { desc } declaration
struct DeclBlock<'code> {
    desc: Vec<DescComment<'code>>,
    decl: Declaration<'code>,
}

struct Const;
struct Func;

async fn compile_func_declaration_block(declarations: FunDeclarations<'_>) -> Result<(), String> {
    println!("---functions---");
    println!("number of functions: {}", declarations.len());

    for decl in declarations {
        compile_func_declaration(decl).await?;
        // tokio::task::spawn(compile_func_declaration(decl))
        //     .await
        //     .unwrap()?;
    }

    Ok(())
}

async fn compile_func_declaration(decl: Block<'_>) -> Result<(), String> {
    match decl {
        Block::EnumDecl(enum_decl) => Err(format!(
            "Block::EnumDecl not used in schema for functions: {:?}",
            enum_decl
        )),
        Block::DescDecl(desc_decl) => {
            let DeclBlock { desc, decl } = desc_decl;
            let meta = desc.compile(()).await?;
            match decl {
                // combinators as object
                Declaration::Combinator(decl) => {
                    Compile::<Func>::compile(decl, meta).await?;
                    Ok(())
                }

                // Base combinators
                Declaration::BuiltinCombinator(decl) => Err(format!(
                    "Declaration::BuiltinCombinator not used in schema for functions: {:?}",
                    decl
                )),

                // Unimplemented
                Declaration::PartialAppDecl(decl) => Err(format!(
                    "Declaration::PartialAppDecl not used in schema for functions: {:?}",
                    decl
                )),
                Declaration::FinalDecl(decl) => Err(format!(
                    "Declaration::FinalDecl not used in schema for functions: {:?}",
                    decl
                )),
            }
        }
    }
}

async fn compile_constr_declaration_block(
    declarations: ConstrDeclarations<'_>,
) -> Result<(), String> {
    println!("---types---");
    println!("number of types: {}", declarations.len());

    for decl in declarations {
        compile_constr_declaration(decl).await?;
        // tokio::task::spawn(compile_constr_declaration(decl))
        //     .await
        //     .unwrap()?;
    }

    Ok(())
}

async fn compile_constr_declaration(decl: Block<'_>) -> Result<(), String> {
    match decl {
        Block::EnumDecl(enum_decl) => enum_decl.compile(()).await,
        Block::DescDecl(desc_decl) => {
            let DeclBlock { desc, decl } = desc_decl;
            let meta = desc.compile(()).await?;
            match decl {
                // combinators as object
                Declaration::Combinator(decl) => {
                    Compile::<Const>::compile(decl, meta).await?;
                    Ok(())
                }

                // Base combinators
                Declaration::BuiltinCombinator(decl) => decl.compile(()).await,

                // Unimplemented
                Declaration::PartialAppDecl(decl) => Err(format!(
                    "Declaration::PartialAppDecl not used in schema for types: {:#?}",
                    decl
                )),
                Declaration::FinalDecl(decl) => Err(format!(
                    "Declaration::FinalDecl not used in schema for types: {:#?}",
                    decl
                )),
            }
        }
    }
}

/// # Compile
/// This is the main entrypoint into the types generator. It takes an input &str and writes the
/// output to the output file at `output_file`
pub fn compile<Output: std::convert::AsRef<std::path::Path>>(
    input: &str,
    output_file: Output,
) -> Result<(), String> {
    let mut buffer = Buffer::from(input);
    // Attempt to parse input
    match <TlProgram as Lexer>::lex(&mut buffer) {
        // If the  input is parseable, attempt to compile
        Ok(output) => {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(output.compile(()))?;

            let output_file = std::fs::File::create(output_file)
                .map_err(|e| format!("FILE OPENING ERROR: {e:#?}"))?;

            let mut output_stream = std::io::BufWriter::new(output_file);

            let date = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();

            // Write headers
            writeln!(
                output_stream,
                r#"
// Programatically generated TDLib API types and methods
// Generated on: {date:?}

use serde::Serialize;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::*;

"#
            )
            .map_err(|e| format!("WRITE ERROR: {e:#?}"))?;

            // Write objects
            for object in Object::read().iter() {
                // let object = object.to_string();
                write!(output_stream, "{object}").map_err(|e| format!("WRITE ERROR: {e:#?}"))?;
                output_stream
                    .flush()
                    .map_err(|e| format!("WRITE ERROR: {e:#?}"))?;
            }

            // Write classes
            for (type_name, type_data) in Type::read().iter() {
                let type_def = TypeDef(type_name, type_data);
                write!(output_stream, "{type_def}").map_err(|e| format!("WRITE ERROR: {e:#?}"))?;
                output_stream
                    .flush()
                    .map_err(|e| format!("WRITE ERROR: {e:#?}"))?;
            }

            // Write functions
            // Write trait
            writeln!(
                output_stream,
                r#"
pub trait Api {{

    #[allow(async_fn_in_trait)]
    /// Handler to send the `payload` to the [crate::Driver]
    async fn send<Payload: Serialize, Target: DeserializeOwned>(
        &self,
        payload: Payload,
    ) -> ::std::result::Result<Target, crate::Failure>;
"#
            )
            .map_err(|e| format!("WRITE ERROR: {e:#?}"))?;

            // Iterate over functions
            for function in Function::read().iter() {
                write!(output_stream, "{function}").map_err(|e| format!("WRITE ERROR: {e:#?}"))?;
                output_stream
                    .flush()
                    .map_err(|e| format!("WRITE ERROR: {e:#?}"))?;
            }

            // End trait
            writeln!(output_stream, r#"}}"#).map_err(|e| format!("WRITE ERROR: {e:#?}"))?;

            // return ok
            Ok(())
        }

        // else throw error
        Err(e) => Err(format!(
            "PARSE ERROR:\nKind: {:?}\nMatched: {}\nBuffer: {}",
            e.kind,
            e.matched,
            &e.buffer.to_string()[0..128]
        )),
    }
}

#[test]
fn build() {
    // let input = include_str!("../subset.md");
    let input = include_str!("../td_api.md");
    compile(input, "generated.rs").unwrap();
}

// mod generated;

//////////////////////////////////////
//////////////////////////////////////
///              Buider            ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
fn build_types<'program>() -> TlProgram<'program> {
    let code = include_str!("../td_api.md");
    let mut buffer = Buffer::from(code);
    match <TlProgram as Lexer>::lex(&mut buffer) {
        Ok(output) => output,
        Err(e) => {
            println!("ERROR: {:?}", e.kind);
            println!("{}", &e.buffer.to_string()[0..128]);
            println!("matched: {}", e.matched);
            panic!();
        }
    }
}

//////////////////////////////////////
//////////////////////////////////////
///             COMPILE            ///
///              PARSE             ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
mod compile_test;

//////////////////////////////////////
//////////////////////////////////////
///              PARSE             ///
///              TESTS             ///
//////////////////////////////////////
//////////////////////////////////////
#[cfg(test)]
mod parse_tests;
