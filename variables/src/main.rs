use rand::Rng;

fn main() {
    println!("Call Another function.");

    another_function(135);

    println!("Another function finished.");

    print_labeled_measurement(5, 'h');

    let return_val = return_integer();
    println!("The value of return_val is: {}", return_val);

    let secret_number = rand::thread_rng().gen_range(1..101);

    if secret_number < 51 {
        println!("Under the half.");
    } else {
        println!("Over the half.");
    }

    /*
     * ループ
     */
    // loop: ラベル付でbreakポイントを制御可能
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // for文
    let for_array = [10, 20, 30, 40, 50];

    for element in for_array {
        println!("the value is: {}", element);
    }
}

/**
 * メイン処理
 */
fn another_function(args: i32) {
    println!("The value of args is: {}", args);

    /*
     * 変数と可変性
     */
    // NG
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y is: {}", y);    
    }

    println!("The value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    // NG
    //let mut spaces = "   ";
    //spaces = spaces.len();
    println!("The value of spaces's length is: {}", spaces);

    /*
     * データ型
     */
    // 整数型
    // MEMO: チュートリアルではビルドエラーになるらしいが、ならなかった
    //       rustup 1.27.1 (2024-04-24)
    // 他言語で言うintegerは"i32"（UNSIGNEDにしたければ"u32"）
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
    // 小数点floatは"f64"
    let floot: f64 = 3.141592;
    println!("The value of floot is: {}", floot);

    // 演算
    // 加算
    let sum = 1 + 2;
    println!("The value of sum is: {}", sum);
    // 減算
    let difference = 4 - 3;
    println!("The value of difference is: {}", difference);
    // 乗算
    let product = 5 * 6;
    println!("The value of product is: {}", product);
    // 除算
    // 他言語と同様、割った時の答えが出る。0も出てくる。
    let quotient = 78.9 / 1.23;
    println!("The value of quotient is: {}", quotient);
    let flooted = 4/5; // 0
    println!("The value of flooted is: {}", flooted);
    // 剰余
    let remainder = 4%5; // 8
    println!("The value of remainder is: {}", remainder);

    /* 
     * bool
     */ 
    let t = true;
    println!("The value of t is: {}", t);
    let f = false;
    println!("The value of f is: {}", f);

    /* 
     * 文字
     */
    let char = 'c';
    println!("The value of char is: {}", char);
    let char2 = '🛜';
    println!("The value of char2 is: {}", char2);
    let string = "string";
    println!("The value of string is: {}", string);

    /*
     * 複合型
     */
    // タプル: 異なる型を織り交ぜてOK
    let tup = (123, 45.6, "789");
    let (col1, col2, col3) = tup;
    println!("The value of col1 is: {}", col1);
    println!("The value of col2 is: {}", col2);
    println!("The value of col3 is: {}", col3);

    println!("The value of tup1 is: {}", tup.0);
    println!("The value of tup2 is: {}", tup.1);
    println!("The value of tup3 is: {}", tup.2);

    // 配列: 全要素同じ型でなければならない
    let array = [1, 2, 3, 4, 5];
    println!("The value of array col1 is: {}", array[0]);
    // NG
    // index out of bounds
    //println!("The value of array col1 is: {}", array[10]);
}

/**
 * 引数複数処理
 */
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn return_integer() -> i32 {
    return 5;
}