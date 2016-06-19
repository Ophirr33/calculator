use std::vec::Vec;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

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
    Left,
    Right
}

fn find_prec(token: &Token) -> usize {
    match token {
        &Token::Num(_) => 0,
        &Token::Add  | &Token::Sub => 1,
        &Token::Mul  | &Token::Div | &Token::Mod => 2,
        &Token::Exp  | &Token::Log => 3,
        &Token::Left | &Token::Right => 4
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        find_prec(&self) == find_prec(other)
    }
}

impl Eq for Token {}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        find_prec(&self).partial_cmp(&find_prec(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        find_prec(&self).cmp(&find_prec(other))
    }
}

fn tokenize(string: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut last_word = String::new();
    for c in string.chars() {
        if c.is_numeric() || c == '.' {
            last_word.push(c);
        } else if is_token(c) {
            add_number(&mut result, &mut last_word);
            add_token(&mut result, c);
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
    match c {
        '+' | '-' | '*' | '/' | '%' | '^' | '_' | '(' | ')' => true,
        _ => false
    }
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
            Token::Num(_) => result.push(token),
            Token::Left   => stack.push(token),
            Token::Right  => {
                while !stack.is_empty() {
                    let item = stack.pop().unwrap();
                    if item == Token::Left {
                        break;
                    } else {
                        result.push(item);
                    }
                    if stack.is_empty() {
                        // At this point, we've found no matching left Paren
                        panic!("Right Parentheses without corresponding Left");
                    }
                }
            },
            _ => token_push_help(token, &mut stack, &mut result)
        }

    }
    stack.reverse();
    result.append(&mut stack);
    result
}

// This function is ugly.
fn token_push_help(token : Token, stack: &mut Vec<Token>, result: &mut Vec<Token>) {
    if stack.is_empty() || *stack.last_mut().unwrap() == Token::Left {
        stack.push(token);
    } else {
        let item = stack.pop().unwrap();
        if token > item {
            stack.push(item);
            stack.push(token);
        } else if token == item {
            result.push(item);
            stack.push(token);
        } else {
            result.push(item);
            token_push_help(token, stack, result);
        }
    }
}

fn eval_postfix(mut tokens : Vec<Token>) -> Option<f64> {
    if tokens.is_empty() { return None; }
    tokens.reverse();
    let mut result = Vec::new();
    while !tokens.is_empty() {
        match tokens.pop().unwrap() {
            Token::Num(n) => result.push(n),
            Token::Add => {
                let n = result.pop().unwrap() + result.pop().unwrap();
                result.push(n);
            },
            Token::Sub => {
                let n1 = result.pop().unwrap();
                let n2 = result.pop().unwrap();
                result.push(n2 - n1);
            },
            Token::Mul => {
                let n = result.pop().unwrap() * result.pop().unwrap();
                result.push(n);
            },
            Token::Div => {
                let n1 = result.pop().unwrap();
                let n2 = result.pop().unwrap();
                result.push(n2 / n1);
            },
            Token::Mod => {
                let n1 = result.pop().unwrap();
                let n2 = result.pop().unwrap();
                result.push(n2 % n1);
            },
            Token::Exp => {
                let n1 = result.pop().unwrap();
                let n2 = result.pop().unwrap();
                result.push(n2.powf(n1));
            },
            Token::Log => {
                let n1 = result.pop().unwrap();
                let n2 = result.pop().unwrap();
                result.push(n2.log(n1));
            },
            _ => panic!("Postfix expressions don't have parentheses")
        }
    }
    result.pop()
}


fn main() {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Err(e) => println!("{}", e),
            _ => ()
        }
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let n = eval_postfix(infix_to_postfix(tokenize(&buffer))).unwrap();
                println!("{}", n);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

#[test]
fn parse_test() {
    //println!("{:?}", tokenize("1+2*(3_10/(10.222222^4))"));
    //println!("{:?}", infix_to_postfix(tokenize("(10 + 7) * 4 - 3^2")));
    let eval = |string| eval_postfix(infix_to_postfix(tokenize(string)));
    let tokens = vec![Token::Num(10.0), Token::Mul, Token::Left, Token::Num(1.0), Token::Add,
                      Token::Num(2.0), Token::Mul, Token::Num(3.0), Token::Right, Token::Add,
                      Token::Num(4.0)];
    let token2 = vec![Token::Num(10.0), Token::Num(2.0), Token::Num(3.0), Token::Mul, Token::Add];
    assert_eq!(tokenize("10 * (1 + 2 * 3) + 4"), tokens);
    assert_eq!(infix_to_postfix(tokenize("10 + 2 * 3")), token2);
    println!("{:?}", infix_to_postfix(tokenize("10 * (1 + 2 * 3) + 4")));
    assert_eq!(eval("10 * (1 + 2 * 3) + 4"), Some(74.0));
    println!("{:?}", infix_to_postfix(tokenize("10 + 2 * 3")));
    assert_eq!(eval("10 + 2 * 3"), Some(16.0));
}
