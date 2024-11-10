use super::*;

#[test]
fn desc() {
    let code = r#"@description An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user"#;

    TestBuild::<Desc>::new(code).pass(None)(|_, _| {});
}

#[test]
fn comment_line() {
    let code = r#"//@description An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
//@phone_number_prefix Prefix of the phone number from which the call will be made
//@length Number of digits in the code, excluding the prefix
authenticationCodeTypeMissedCall phone_number_prefix:string length:int32 = AuthenticationCodeType;"#;

    TestBuild::<DescComment>::new(code).pass(None)(|_, _| {});
}

#[test]
fn comment_class() {
    let code = r#"//@class AuthenticationCodeType @description Provides information about the method by which an authentication code is delivered to the user
"#;

    TestBuild::<ClassDesc>::new(code).pass(None)(|_, _| {});
}

#[test]
fn multi_line_comment() {
    let code = r#"//@description An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
//ugh
//@phone_number_prefix Prefix of the phone number from which the call will be made
//@length Number of digits in the code, excluding the prefix
authenticationCodeTypeMissedCall phone_number_prefix:string length:int32 = AuthenticationCodeType;"#;

    TestBuild::<Vec<DescComment>>::new(code).pass(None)(|_, _| {});
}
