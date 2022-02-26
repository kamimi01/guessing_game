use std::io;  // 標準入力用
use std::cmp::Ordering;  // Less/Greater/Equalの列挙型
use rand::Rng;  // 乱数生成用

fn main() {
    println!("数を当ててごらん");

    // 乱数生成
    let secret_number = rand::thread_rng().gen_range(1, 11);

    // println!("秘密の数字は次の通り: {}", secret_number);

    loop {
        println!("予想を入力してね");

        // mutをつけることで変数になる
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");
    
        // guessという変数はあるが、Rustのシャドーイングという機能によって、値を別の型に変換できる
        // trim()で両端の空白を除去する
        // parse()で文字列を解析してなんらかの数値にする
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("次のように予想しました: {}", guess);
    
        // cmpメソッドは、2値を比較する
        // matchは比較用の式
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("やったね！");
                break;
            }
        }
    }
}
