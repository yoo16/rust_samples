fn main() {
    println!("Hello, world!");

    calculate();
    show_types();
    show_score();
    show_counts();
    show_ownership();

    let s = String::from("Hello");
    show_length(&s); // 所有権は移動しない
    println!("{}", s); // sはまだ使える
}

fn calculate() {
    let x = 5; // 不変
    let mut y = 10; // 可変

    println!("x = {}, y = {}", x, y);

    y = 20; // OK
    println!("更新後: y = {}", y);
}

fn show_types() {
    let a: i32 = 42; // 整数
    let b: f64 = 3.14; // 浮動小数点
    let c: bool = true; // 真偽値
    let d: char = 'A'; // 文字
    let s1: &str = "Rust"; // 文字列スライス
    let mut s2 = String::from("Hello, "); // 可変なString
    s2.push_str("Tokyo!"); // 可変なStringに文字列を追加

    println!("a={}, b={}, c={}, d={}, s1={}, s2={}", a, b, c, d, s1, s2);
}

fn show_score() {
    let score = 85;

    if score >= 90 {
        println!("Excellent!");
    } else if score >= 70 {
        println!("Good!");
    } else {
        println!("Keep trying!");
    }
}

fn show_counts() {
    // forループ
    for i in 1..=5 {
        println!("i = {}", i);
    }

    // whileループ
    let mut count = 0;
    while count < 3 {
        println!("count = {}", count);
        count += 1;
    }
}

fn show_ownership() {
    let s1 = String::from("Rust");
    let s2 = s1; // 所有権がs2に移動（ムーブ）

    // println!("{}", s1); // ❌ エラー：s1の所有権は既に移動済み
    println!("{}", s2); // OK
}

fn show_length(text: &String) {
    println!("文字列の長さ: {}", text.len());
}