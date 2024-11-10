use super::*;

#[test]
fn desc_decl() {
    let code = r#"//@description An object of this type can be returned on every function call, in case of an error
//@code Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
//@message Error message; subject to future changes
error code:int32 message:string = Error;"#;
    TestBuild::<DeclBlock>::new(code).pass(None)(|_, _| {});
}

#[test]
fn block_desc_decl() {
    let code = r#"//@description An object of this type can be returned on every function call, in case of an error
//@code Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
//@message Error message; subject to future changes
error code:int32 message:string = Error;"#;
    TestBuild::<Block>::new(code).pass(None)(|_, _| {});
}

#[test]
fn block_full() {
    let code = r#"double ? = Double;
string ? = String;

int32 = Int32;
int53 = Int53;
int64 = Int64;
bytes = Bytes;

boolFalse = Bool;
boolTrue = Bool;

vector {t:Type} # [ t ] = Vector t;


//@description An object of this type can be returned on every function call, in case of an error
//@code Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
//@message Error message; subject to future changes
error code:int32 message:string = Error;


//@description An object of this type is returned on a successful function call for certain functions
ok = Ok;


//@class AuthenticationCodeType @description Provides information about the method by which an authentication code is delivered to the user

//@description A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
//@length Length of the code
authenticationCodeTypeTelegramMessage length:int32 = AuthenticationCodeType;"#;
    TestBuild::<Vec<Block>>::new(code).pass(None)(|_, _| {});
}

#[test]
fn complex_block() {
    let code = r#"//@description Contains a list of text entities @entities List of text entities
textEntities entities:vector<textEntity> = TextEntities;"#;
    TestBuild::<Block>::new(code).pass(None)(|_, _| {});
}

#[test]
fn full_prog() {
    let output = build_types();
    println!("SUCCESS:");
    std::fs::write("ir", &format!("{output:#?}")).unwrap();
}
