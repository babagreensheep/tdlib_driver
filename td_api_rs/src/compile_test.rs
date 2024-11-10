use super::*;

#[cfg(test)]
fn build_subset<'program>() -> TlProgram<'program> {
    let code = include_str!("../subset.md");
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

#[test]
fn compile_const_str() {
    let program = build_types();

    compile::harness(program.compile(())).unwrap();

    std::fs::write("functions", format!("{:#?}", Function::read())).unwrap();
    std::fs::write("types", format!("{:#?}", Type::read())).unwrap();
    std::fs::write("objects", format!("{:#?}", Object::read())).unwrap();
}
