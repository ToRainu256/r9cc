fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: i32 = args[1].parse().unwrap();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("    mov rax, {}", input);
    println!("    ret");
}
