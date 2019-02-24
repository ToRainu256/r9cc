extern crate r9cc;
use r9cc::strtol;

fn main() {
    //標準入力を受け取り、引数の個数をチェック
    let mut args = std::env::args();
    //println!("{:?}", args);
    if args.len() != 2 {
        eprint!("Usage: 9cc <code>\n");
        return;
    }
    //nth(n)はn番目の文字列を取り出す
    let p = args.nth(1).unwrap();

    //最初のおまじない
    //INTEL記法のアセンブリを使うことを明示したあと
    //MAIN関数を定義する
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let (n, mut p) = strtol(&p); //C言語のstrtolに相当文字列をlong型の整数に変換する
    println!("    mov rax, {}", n.unwrap());
    //自作のstrtolは引数の変換した部分を切り取ったものを返す
    //だから、０番目から舐める。
    //
    while let Some(c) = p.chars().nth(0) {
        let s = p.split_off(1); //Vecのｎ番目以降を切り落とす。

        if c == '+' {
            let (n, remaining) = strtol(&s);
            p = remaining;
            println!("    add rax, {}", n.unwrap());
            continue;
        }

        if c == '-' {
            let (n, remaining) = strtol(&s);
            p = remaining;
            println!("    sub rax, {}", n.unwrap());
            continue;
        }

        eprint!("unexpected character: {}\n", p);
        return;
    }
    println!("    ret");
    return;
}
