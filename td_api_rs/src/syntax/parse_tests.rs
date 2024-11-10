use super::*;

#[test]
fn term() {
    let code = "Int";
    TestBuild::<Term>::new(code).pass(None)(|_, _| {});
}

#[test]
fn subexpr_term() {
    let code = "Int";
    TestBuild::<Subexpr>::new(code).pass(None)(|_, _| {});
}

#[test]
fn subexpr_nat_const_subexpr() {
    let code = "100 + Int";
    TestBuild::<Subexpr>::new(code).pass(None)(|_, _| {});
}

#[test]
fn expr() {
    let code = "10";
    TestBuild::<Expr>::new(code).pass(None)(|_, _| {});
}

#[test]
fn angle_brace() {
    let code = r#"vector<textEntity>"#;
    TestBuild::<Term>::new(code).pass(None)(|_, _| {});
}
