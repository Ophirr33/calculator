use std::collections::HashMap;
use std::vec::Vec;

enum Expr<'a> {
    Num(f64),
    Id(&'a str),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
    Mod(Box<Expr<'a>>, Box<Expr<'a>>),
    Exp(Box<Expr<'a>>, Box<Expr<'a>>),
    Log(Box<Expr<'a>>, Box<Expr<'a>>),
}

#[derive (Debug)]
enum Token {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Log,
    LeftParentheses,
    RightParentheses,
    Equal
}

fn main() {
    println!("Sup!");
}

fn tokenize(string: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut last_word = String::new();
    for c in string.chars() {
        if c.is_numeric() || c == '.' {
            last_word.push(c);
        } else if is_token(c) {
            add_token(&mut result, c);
            add_number(&mut result, &mut last_word);
        } else if c.is_whitespace() {
            add_number(&mut result, &mut last_word);
        } else {
            panic!("Could not tokenize the given string");
        }
    }
    add_number(&mut result, &mut last_word);
    return result;
}

fn is_token(c: char) -> bool {
    vec!['+', '-', '*', '/', '%', '^', '_', '(', ')', '='].iter().any(|&token| token == c)
}

fn add_token(vec: &mut Vec<Token>, c: char) {
    let token = match c {
        '+' => Token::Add,
        '-' => Token::Sub,
        '*' => Token::Mul,
        '/' => Token::Div,
        '%' => Token::Mod,
        '^' => Token::Exp,
        '_' => Token::Log,
        '(' => Token::LeftParentheses,
        ')' => Token::RightParentheses,
        '=' => Token::Equal,
        _ => panic!("Not given a valid token character")
    };
    vec.push(token);
}

fn add_number(vec: &mut Vec<Token>, word: &mut String) {
    if !word.is_empty() {
        match word.parse::<f64>() {
            Ok(num) => {
                vec.push(Token::Num(num));
                word.clear();
            }
            _ => panic!("Not a given valid token creator")
        }
    }
}

fn is_numeric(string: &str) -> bool {
    let mut period_count = 0;
    let mut last = 'z';
    for c in string.chars() {
        last = c;
        if c == '.' { period_count += 1 }
        else if !c.is_numeric() { return false }
    }
    last.is_numeric() && period_count < 2
}

//fn parse(tokens: Vec<Token>) -> Expr

impl<'a> Expr<'a> {
    fn eval(&self, env: &HashMap<&str, f64>) -> f64 {
        match self{
            &Expr::Num(n) => n,
            &Expr::Id(key) => *(env.get(key).unwrap()),
            &Expr::Add(ref left, ref right) => (**left).eval(env) + (**right).eval(env),
            &Expr::Sub(ref left, ref right) => (**left).eval(env) - (**right).eval(env),
            &Expr::Mul(ref left, ref right) => (**left).eval(env) * (**right).eval(env),
            &Expr::Div(ref left, ref right) => (**left).eval(env) / (**right).eval(env),
            &Expr::Mod(ref left, ref right) => (**left).eval(env) % (**right).eval(env),
            &Expr::Exp(ref left, ref right) => (**left).eval(env).powf((**right).eval(env)),
            &Expr::Log(ref left, ref right) => (**left).eval(env).log((**right).eval(env))
        }
    }
}


#[test]
fn parse_test() {
    let four = "-4.000000".parse::<f64>().unwrap();
    assert!(four.is_normal());
    let thirty_five = Expr::Add(Box::new((Expr::Id("a"))), Box::new((Expr::Num(18.0))));
    let mut hash_map : HashMap<&str, f64> = HashMap::new();
    hash_map.insert("a", 17.0);
    assert_eq!(thirty_five.eval(&hash_map), 35.0);
    assert!(is_numeric("22.25"));
    assert!(is_numeric(".2225"));
    assert!(!is_numeric("2225."));
    println!("{:?}", tokenize("1+2*(3_10/(10.222222^4))"));
}
