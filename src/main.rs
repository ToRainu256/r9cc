extern crate r9cc;
use r9cc::strtol;

use std::env;
use std::process::exit;

enum TokenType {
    Num,
}
#[derive(Default, Debug)]
struct Token {
    ty: i32,       //TokenType
    val: i32,      //Number literal
    input: String, //Token string (error)
}

fn tokenize(mut p: String) -> Vec<Token> {
    //トークナイズした文字列はtokensに保管
    let mut tokens: Vec<Token> = vec![];

    let org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        if c.is_whitespace() {
            p = p.split_off(1); //pを１つずらす
            continue;
        }

        //+ or -
        if c == '+' || c == '-' {
            let token = Token {
                ty: c as i32,
                input: org.clone(),
                ..Default::default()
            };
            p = p.split_off(1);
            tokens.push(token);
            continue;
        }

        //Number
        if c.is_ascii_digit() {
            let (n, remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                ty: TokenType::Num as i32,
                input: org.clone(),
                val: n.unwrap() as i32,
            };
            tokens.push(token);
            continue;
        }

        eprint!("cannot tokenize: {}\n", p);
        exit(1);
    }
    return tokens;
}

fn fail(tokens: &Vec<Token>, i: usize) {
    eprintln!("unexpected character: {:?}", tokens[i]);
    exit(1);
}

fn main() {
    //標準入力を受け取り、引数の個数をチェック
    let mut args = std::env::args();
    //println!("{:?}", args);
    if args.len() != 2 {
        eprint!("Usage: 9cc <code>\n");
        return;
    }
    //標準入力を関数tokenizeに渡す
    let tokens = tokenize(args.nth(1).unwrap());

    //最初のおまじない
    //INTEL記法のアセンブリを使うことを明示したあと
    //MAIN関数を定義する
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    //最初の入力が数値でなければエラー
    //例えば、+か-
    if tokens[0].ty != TokenType::Num as i32 {
        fail(&tokens, 0);
    }
    println!("    mov rax, {}", tokens[0].val);

    let mut i = 1;
    while i != tokens.len() {
        if tokens[i].ty == '+' as i32 {
            i += 1;
            if tokens[i].ty != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            print!("    add rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        if tokens[i].ty == '-' as i32 {
            i += 1;
            if tokens[i].ty != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            println!("    sub rax, {}", tokens[i].val);
            i += 1;
            continue;
        }

        fail(&tokens, i);
    }
    println!("    ret");
}
