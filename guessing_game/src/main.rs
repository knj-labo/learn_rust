use std::io;
// Rngトレイトは乱数生成器が実装すべきメソッドを定義
use rand::Rng;

fn main() {

    // println! は画面に文字列を表示するマクロ
    println!("Guess the number!"); // 数を当ててごらん
    // 乱数生成して 開始〜終了
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}

    println!("Please input your guess."); // ほら、予想を入力してね

    // let を使用して変数を宣言
    // Rust は変数を可変（mutable）にするには、`mut`をつける
    // let apples = 5;      // immutable
    // let mut bananas = 5; // mutable
    // 可変変数を作成し、その変数は現時点では新しい空のStringのインスタンスに束縛されている
    let mut guess = String::new();

    // `stdin関数`はターミナルの標準入力へのハンドルを表す型である std::io::Stdin のインスタンスを返す
    // read_lineメソッドは、ユーザが標準入力に入力したものを文字列に追加すること
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("You guessed: {}", guess);     // 次のように予想しました: {}
}
