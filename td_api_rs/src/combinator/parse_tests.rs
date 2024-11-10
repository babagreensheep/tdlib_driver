use super::*;

#[test]
fn combinator_decl_test() {
    let code = r#"int32 = Int32;
int53 = Int53;
int64 = Int64;
bytes = Bytes;

boolFalse = Bool;
boolTrue = Bool;
vector {t:Type} # [ t ] = Vector t;
"#;
    TestBuild::<Vec<CombinatorDecl>>::new(code).pass(None)(|_, _| {});
}

#[test]
fn combinator_vec() {
    let code = r#"textEntities entities:vector<textEntity> = TextEntities;"#;
    TestBuild::<CombinatorDecl>::new(code).pass(None)(|_, _| {});
}
