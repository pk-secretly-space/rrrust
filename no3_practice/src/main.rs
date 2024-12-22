use rand::Rng;

fn main() {
    degree();
    fibonacci();
    carol();
}

/**
 * 温度を摂氏と華氏に変換する処理
 * ・基準は摂氏（℃）
 * ・摂氏はランダム数値、華氏は摂氏から計算で導く。
 */
fn degree() {
    let celsius = rand::thread_rng().gen_range(0..101);
    let fahrenheit = celsius * 9 / 5 + 32;
    println!("Degree celsius is {} and fahrenheit is {}", celsius, fahrenheit);
}

/**
 * フィボナッチ数列のn番目を出力する処理
 * ・n番目はランダム変数で10番目まで。
 * ・本当はもっとスマートにできるが、ループのお勉強としてここではこの形に。
 */
fn fibonacci() {
    let index = rand::thread_rng().gen_range(1..11);
    let mut count = 1;

    let mut first = 0;
    let mut second = 1;
    let mut fibonacci = 0;
    loop {
        if count == index {
            break;
        }
        fibonacci = first + second;
        first = second;
        second = fibonacci;

        count += 1;
    }

    println!("Fibonacci index {} value is {}.", index, fibonacci);
}

fn carol() {
    let index = rand::thread_rng().gen_range(0..13);
    println!("On the first day of Christmas my true love sent to me");
    if index == 0 {
        println!("a Partridge in a Pear Tree.");
        return;
    }
    let mut count = index;
    let lyrics = [
        "and a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings.",
        "six geese a-laying,",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    
    loop {
        println!("{}", lyrics[count]);
        if count == 0 {
            break;
        }
        count -= 1;
    }
}