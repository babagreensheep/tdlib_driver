use super::*;

#[derive(Debug, Clone, Lexer, Token)]
/// lc-letter ::= a | b | ... | z
pub struct LcLetter<'code>(
    #[pattern(
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z"
    )]
    Buffer<'code>,
);

#[derive(Debug, Clone, Lexer, Token)]
/// uc-letter ::= A | B | ... | Z
pub struct UcLetter<'code>(
    #[pattern(
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z"
    )]
    Buffer<'code>,
);

#[derive(Debug, Clone, Lexer, Token)]
/// digit ::= 0 | 1 | ... | 9
pub struct Digit<'code>(#[pattern("0", "1", "2", "3", "4", "5", "6", "7", "8", "9")] Buffer<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// hex-digit ::= digit | a | b | c | d | e | f
pub enum HexDigit<'code> {
    Digit(Digit<'code>),
    HexLetter(#[pattern("a", "b", "c", "d", "e", "f")] Buffer<'code>),
}

/// hex-code ::= # hex-digit *8
#[derive(Debug, Clone, Lexer, Token)]
pub struct HexCode<'code>(Hash<'code>, [HexDigit<'code>; 8], Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// underscore ::= _
pub struct Underscore<'code>(#[pattern("_")] Buffer<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// letter ::= lc-letter | uc-letter
pub enum Letter<'code> {
    Lc(LcLetter<'code>),
    Uc(UcLetter<'code>),
}

#[derive(Debug, Clone, Lexer, Token)]
/// ident-char ::= letter | digit | underscore
pub enum IdentChar<'code> {
    Letter(Letter<'code>),
    Digit(Digit<'code>),
    Underscore(Underscore<'code>),
}

#[derive(Clone, Lexer, Token)]
/// lc-ident ::= lc-letter { ident-char }
pub struct LcIdent<'code>(
    LcLetter<'code>,
    Group<'code, IdentChar<'code>, 0>,
    Spaces<'code>,
);

impl<'code> std::fmt::Display for LcIdent<'code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ident = self.buffer().map(|b| b.to_string()).unwrap_or("".into());
        std::fmt::Display::fmt(&ident, f)
    }
}

impl<'code> std::fmt::Debug for LcIdent<'code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[derive(Clone, Lexer, Token)]
/// uc-ident ::= uc-letter { ident-char }
pub struct UcIdent<'code>(
    UcLetter<'code>,
    Group<'code, IdentChar<'code>, 0>,
    Spaces<'code>,
);

impl<'code> std::fmt::Display for UcIdent<'code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ident = self.buffer().map(|b| b.to_string()).unwrap_or("".into());
        std::fmt::Display::fmt(&ident, f)
    }
}

impl<'code> std::fmt::Debug for UcIdent<'code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

/// namespace-ident ::= lc-ident
pub type NamespaceIdent<'code> = LcIdent<'code>;

#[derive(Debug, Clone, Lexer, Token)]
/// lc-ident-ns ::= [ namespace-ident . ] lc-ident
pub struct LcIdentNs<'code> {
    pub namespace: Option<Pair<'code, NamespaceIdent<'code>, Dot<'code>>>,
    pub ident: LcIdent<'code>,
}

impl<'code> Compile<LcIdent<'code>> for LcIdentNs<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<LcIdent<'code>, String> {
        let _ = state;
        // Throw error for name space
        if let Some(Pair(namespace, ..)) = self.namespace {
            Err(format!("name space ({namespace}) not supported"))?;
        }
        Ok(self.ident)
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// uc-ident-ns ::= [ namespace-ident . ] uc-ident
pub struct UcIdentNs<'code> {
    pub namespace: Option<Pair<'code, NamespaceIdent<'code>, Dot<'code>>>,
    pub type_name: UcIdent<'code>,
}

impl<'code> Compile<UcIdent<'code>> for UcIdentNs<'code> {
    type State = ();

    async fn compile(self, state: Self::State) -> Result<UcIdent<'code>, String> {
        let _ = state;
        // Throw error for name space
        if let Some(Pair(namespace, ..)) = self.namespace {
            Err(format!("name space ({namespace}) not supported"))?;
        }
        Ok(self.type_name)
    }
}

#[derive(Debug, Clone, Lexer, Token)]
/// lc-ident-full ::= lc-ident-ns [ # hex-digit *8 ]
pub struct LcIdentFull<'code>(pub LcIdentNs<'code>, pub Option<HexCode<'code>>);

#[derive(Debug, Clone, Lexer, Token)]
/// colon ::= :
pub struct Colon<'code>(#[pattern(":")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// semicolon ::= ;
pub struct Semicolon<'code>(#[pattern(";")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// open-par ::= (
pub struct OpenPar<'code>(#[pattern("(")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// close-par ::= )
pub struct ClosePar<'code>(#[pattern(")")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// open-bracket ::= [
pub struct OpenBracket<'code>(#[pattern("[")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// close-bracket ::= ]
pub struct CloseBracket<'code>(#[pattern("]")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// open-brace ::= {
pub struct OpenBrace<'code>(#[pattern("{")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// close-brace ::= }
pub struct CloseBrace<'code>(#[pattern("}")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// triple-minus ::= ---
pub struct TripleMinus<'code>(#[pattern("---")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// nat-const ::= digit { digit }
pub struct NatConst<'code>(Digit<'code>, Group<'code, Digit<'code>, 0>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// equals ::= =
pub struct Equals<'code>(#[pattern("=")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// hash ::= #
pub struct Hash<'code>(#[pattern("#")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// question-mark ::= ?
pub struct QuestionMark<'code>(#[pattern("?")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// percent ::= %
pub struct Percent<'code>(#[pattern("%")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// plus ::= +
pub struct Plus<'code>(#[pattern("+")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// langle ::= <
pub struct Langle<'code>(#[pattern("<")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// rangle ::= >
pub struct Rangle<'code>(#[pattern(">")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// comma ::= ,
pub struct Comma<'code>(#[pattern(",")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// dot ::= .
pub struct Dot<'code>(#[pattern(".")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// asterisk ::= *
pub struct Asterisk<'code>(#[pattern("*")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// excl-mark ::= !
pub struct ExclMark<'code>(#[pattern("!")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// Final-kw ::= Final
pub struct FinalKw<'code>(#[pattern("Final")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// New-kw ::= New
pub struct NewKw<'code>(#[pattern("New")] Buffer<'code>, Spaces<'code>);

#[derive(Debug, Clone, Lexer, Token)]
/// Empty-kw ::= Empty
pub struct EmptyKw<'code>(#[pattern("Empty")] Buffer<'code>, Spaces<'code>);

#[test]
fn uc_ident_test() {
    let code = "Int";

    TestBuild::<UcIdent>::new(code).pass(None)(|_, _| {});
}
