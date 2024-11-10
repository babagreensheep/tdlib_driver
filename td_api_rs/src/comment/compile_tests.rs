use super::*;

#[test]
fn enum_decl() {
    let code = r#"//@class AuthenticationCodeType @description Provides information about the method by which an authentication code is delivered to the user"#;

    let enum_decl = <ClassDesc as Lexer>::lex(&mut Buffer::from(code)).unwrap();

    compile::harness(enum_decl.compile(())).unwrap();

    println!("{TYPES:#?}");
}
