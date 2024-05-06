use ::std::io;
use std::iter::Peekable;

// 位置情報（Loc(4, 8) なら 入力文字の5文字目から9文字目までの範囲を表す）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

// アノテーション。値に位置情報を持たせる
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}
impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Annot { value, loc }
    }
}

struct MyError {}
struct NewMyError {}

impl From<MyError> for NewMyError {
    fn from(_: MyError) -> Self {
        NewMyError {}
    }
}

fn test() -> Result<(), NewMyError> {
    let e = Err(MyError {});
    e?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

type Token = Annot<TokenKind>;
impl Token {
    //型エイリアスにも実装つけられる
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc) // Token { value: TokenKind::Number(n), loc }
    }
    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }
    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;
impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}

fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = vec![];
    let input = input.as_bytes(); //バイト列&[u8]を受け取る

    let mut pos = 0;

    // lex_a_token! マクロを定義、マクロはコードを生成するための機能
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        match input[pos] {
            b'0'..=b'9' => lex_a_token!(lex_number(input, pos)),
            //b'+' はバイト文字リテラル、ASCII文字コードのみ対応　b'+' は &[u8; 1]型
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
            _ => return Err(LexError::eof(Loc(0, 0))),
        }
    }
    Ok(tokens)
}

// pos のバイトが期待するものなら、1バイト（ASCIIだから） consume して pos を1進める
fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    println!("{}, {}", input.len(), pos);

    if input.len() <= pos {
        return Err(LexError::eof(Loc(pos, pos)));
    }

    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
    //okの場合、トークンを作成して返す
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Loc(start, end)), end))
}

fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Loc(start, end)), end))
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}

fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

fn lex_number(input: &[u8], mut pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;

    let start = pos;
    let end = recognize_many(input, pos, |b| b"123456789".contains(&b));

    let n = from_utf8(&input[start..end])
        .unwrap()
        .parse::<u64>()
        .unwrap();

    Ok((Token::number(n, Loc(start, end)), end))
}

fn skip_spaces(input: &[u8], mut pos: usize) -> Result<((), usize), LexError> {
    let end = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((), end))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AstKind {
    Num(u64),
    UniOp { op: UniOp, e: Box<Ast> },              //単項演算
    BinOp { op: BinOp, l: Box<Ast>, r: Box<Ast> }, //二項演算
} // 木構造を表す

// ex:
// 1 + 2 * 3

type Ast = Annot<AstKind>;
impl Ast {
    fn num(n: u64, loc: Loc) -> Self {
        Self::new(AstKind::Num(n), loc)
    }
    fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
        Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }
    fn binop(op: BinOp, l: Ast, r: Ast, loc: Loc) -> Self {
        Self::new(
            AstKind::BinOp {
                op,
                l: Box::new(l),
                r: Box::new(r),
            },
            loc,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum UniOpKind {
    Plus,
    Minus,
}
type UniOp = Annot<UniOpKind>;
impl UniOp {
    fn plus(loc: Loc) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(UniOpKind::Minus, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BinOpKind {
    Add,
    Sub,
    Mult,
    Div,
}

type BinOp = Annot<BinOpKind>;
impl BinOp {
    fn add(loc: Loc) -> Self {
        Self::new(BinOpKind::Add, loc)
    }
    fn sub(loc: Loc) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }
    fn mult(loc: Loc) -> Self {
        Self::new(BinOpKind::Mult, loc)
    }
    fn div(loc: Loc) -> Self {
        Self::new(BinOpKind::Div, loc)
    }
}

// 構文解析時のエラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParseError {
    UnExpectedToken(Token),     //予期しないトークンがきた
    NotExpression(Token),       //式を期待してたけど式以外がきた
    NotOperator(Token),         //演算子を期待してたけど演算子以外がきた
    UnclosedOpenParen(Token),   //括弧が閉じられていない
    RedundantExpression(Token), //式の解析が終わったけどトークンが余ってる
    Eof,
}

// LexError, ParseError の列挙型を作成
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Error {
    Lexer(LexError),
    Parser(ParseError),
}

// Error::from で LexError, ParseError を透過的に扱う
// この from は Error 型を返す関数の中で、Result / Option を ? で評価したとき、その結果が LexError 型だった場合に自動的に呼び出される
impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}

// EXPR ;
fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    //iter.peek()可能な iteratorを作る（iter.next() と違いイテレータを消費しないので先読みが可能）

    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ret),
    }
}

// EXPR = EXPR3 ;
fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr3(tokens)
}

// EXPR3 = EXPR2 EXPR3_Loop
// EXPR3_Loop = ("+"|"-") EXPR2 EXPR3_Loop | ε
fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    Ok::<char, ()>('a').ok();

    parse_left_binop(tokens, parse_expr2, |tokens| {
        let op = tokens.peek().map_or_else(
            || Err(ParseError::Eof),
            |tok| match tok {
                Token {
                    value: TokenKind::Plus,
                    loc,
                } => Ok(BinOp::add(loc.clone())),
                Token {
                    value: TokenKind::Minus,
                    loc,
                } => Ok(BinOp::sub(loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            },
        )?; // Err の場合は return して関数を抜ける
        tokens.next();
        Ok(op)
    })

    // let mut e = parse_expr2(tokens)?;

    // loop {
    //     match tokens.peek().map(|tok| tok.value) {
    //         Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
    //             let op = match tokens.next().unwrap() {
    //                 Token {
    //                     value: TokenKind::Plus,
    //                     loc,
    //                 } => BinOp::add(loc),
    //                 Token {
    //                     value: TokenKind::Minus,
    //                     loc,
    //                 } => BinOp::sub(loc),
    //                 _ => unreachable!(),
    //             };
    //             let r = parse_expr2(tokens)?;
    //             let loc = e.loc.merge(&r.loc);
    //             e = Ast::binop(op, e, r, loc)
    //         }
    //         //  ε
    //         _ => return Ok(e),
    //     }
    // }
}

// EXPR2 = EXPR1 EXPR2_Loop
// EXPR2_Loop = ("*"|"/") EXPR1 EXPR2_Loop | ε
fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_left_binop(tokens, parse_expr1, |tokens| {
        let op = tokens
            .peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Asterisk => Ok(BinOp::mult(tok.loc.clone())),
                TokenKind::Slash => Ok(BinOp::div(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;

        tokens.next();
        Ok(op)
    })
    // let mut e = parse_expr1(tokens)?;

    // loop {
    //     match tokens.peek().map(|tok| tok.value) {
    //         Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
    //             let op = match tokens.next().unwrap() {
    //                 Token {
    //                     value: TokenKind::Asterisk,
    //                     loc,
    //                 } => BinOp::mult(loc),
    //                 Token {
    //                     value: TokenKind::Slash,
    //                     loc,
    //                 } => BinOp::div(loc),
    //                 _ => unreachable!(),
    //             };
    //             let r = parse_expr1(tokens)?;
    //             let loc = e.loc.merge(&r.loc);
    //             e = Ast::binop(op, e, r, loc)
    //         }
    //         _ => return Ok(e),
    //     }
    // }
}

// EXPR1 = ("+" | "-"), ATOM | ATOM ;
fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek().map(|tok| tok.value) {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
            let op = match tokens.next() {
                Some(Token {
                    value: TokenKind::Plus,
                    loc,
                }) => UniOp::plus(loc),
                Some(Token {
                    value: TokenKind::Minus,
                    loc,
                }) => UniOp::minus(loc),
                _ => unreachable!(),
            };
            let e = parse_atom(tokens)?;
            let loc = op.loc.merge(&e.loc);
            Ok(Ast::uniop(op, e, loc))
        }
        _ => parse_atom(tokens),
    }
}

// ATOM = UNUMBER | "(", EXPR3, ")" ;
fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|tok| match tok.value {
            TokenKind::Number(n) => Ok(Ast::new(AstKind::Num(n), tok.loc)),
            TokenKind::LParen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::RParen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParseError::RedundantExpression(t)),
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}

fn parse_left_binop<Tokens>(
    tokens: &mut Peekable<Tokens>,
    subexpr_parser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
    op_parser: fn(&mut Peekable<Tokens>) -> Result<BinOp, ParseError>,
) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut e = subexpr_parser(tokens)?;

    loop {
        match tokens.peek() {
            Some(_) => {
                let op = match op_parser(tokens) {
                    Ok(op) => op,
                    Err(_) => break,
                };
                let r = subexpr_parser(tokens)?;
                let loc = e.loc.merge(&r.loc);
                e = Ast::binop(op, e, r, loc)
            }
            _ => break,
        }
    }
    Ok(e)
}

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let token = lex(&line);
            println!("{:?}", token);
        } else {
            break;
        }
    }
}

#[test]
fn test_parser() {
    let ast = parse(lex("1 + 2 * 3").unwrap()).unwrap();
    println!("{:?}", ast);
    assert_eq!(
        ast,
        Ast::binop(
            BinOp::add(Loc(2, 3)),
            Ast::num(1, Loc(0, 1)),
            Ast::binop(
                BinOp::mult(Loc(6, 7)),
                Ast::num(2, Loc(4, 5)),
                Ast::num(3, Loc(8, 9)),
                Loc(4, 9)
            ),
            Loc(0, 9)
        )
    );
    let ast: Annot<AstKind> = parse(lex("(1 + 2) * 3 + 4").unwrap()).unwrap();
    println!("{:?}", ast);
    assert_eq!(
        ast,
        Ast::binop(
            BinOp::add(Loc(12, 13)),
            Ast::binop(
                BinOp::mult(Loc(8, 9)),
                Ast::binop(
                    BinOp::add(Loc(3, 4)),
                    Ast::num(1, Loc(1, 2)),
                    Ast::num(2, Loc(5, 6)),
                    Loc(1, 6)
                ),
                Ast::num(3, Loc(10, 11)),
                Loc(1, 11)
            ),
            Ast::num(4, Loc(14, 15)),
            Loc(1, 15)
        )
    );
}
