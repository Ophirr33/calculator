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
    let four = "4.000000".parse::<f64>().unwrap();
    assert!(four.is_normal());
    let thirty_five = Expr::Add(Box::new((Expr::Id("a"))), Box::new((Expr::Num(18.0))));
    let mut hash_map : HashMap<&str, f64> = HashMap::new();
    hash_map.insert("a", 17.0);
    assert_eq!(thirty_five.eval(&hash_map), 35.0);
    assert!(is_numeric("22.25"));
    assert!(is_numeric(".2225"));
    assert!(!is_numeric("2225."));
}
