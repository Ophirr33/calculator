use std::collections::HashMap;
use std::vec::Vec;

#[derive (Debug, PartialEq)]
enum Token {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Log,
    Left,
    Right
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        let find_prec = |&token| {
            match token {
                &Num(_)             => 0,
                &Add  | &Sub        => 1,
                &Mul  | &Div | &Mod => 2,
                &Exp  | &Log        => 3,
                &Left | &Right      => 4
            }
        }
        find_prec(&self).cmp(find_prec(other))
    }
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
        '(' => Token::Left,
        ')' => Token::Right,
        _   => panic!("Not given a valid token character")
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

// http://csis.pace.edu/~wolf/CS122/infix-postfix.htm
fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack = Vec::new();
    let mut result = Vec::new();
    for token in tokens {
        match token {
            Token::Num(n) => result.push(token),
            Token::Left   => stack.push(token),
            Token::Right  => {
                while !stack.is_empty() {
                    let item = stack.pop.unwrap();
                    if item == Left {
                        break;
                    } else {
                        result.push(item);
                    }
                }
                // At this point, we've found no matching left Paren
                panic!("Right Parentheses without corresponding Left");
            },
            _ => token_push_help(&token, &mut stack, &mut result)
        }
    }
    result.append(stack);
    result
}

// This function is ugly.
fn token_push_help(token :: &Token, stack: &mut Vec<Token>, result: &mut Vec<Token>) {
    if stack.is_empty() {
        stack.push(token);
    } else {
        let mut item = stack[stack.size() - 1];
        if token > item {
            stack.push(item);
        } else if token == item {
            result.push(stack.pop().unwrap());
            stack.push(token);
        } else {
            result.push(stack.pop().unwrap());
            token_push_help(token, stack, result);
        }
    }
}


#[test]
fn parse_test() {
    let four = "-4.000000".parse::<f64>().unwrap();
    assert_eq!(four, -4);
    println!("{:?}", tokenize("1+2*(3_10/(10.222222^4))"));
    println!("{:?}", infix_to_postfix(tokenize("(10 + 7) * 4 - 3^2")));
}
