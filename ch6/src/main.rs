use std::io;
use std::io::Write;

struct Circle {
    radius: f64,
}

impl Circle {
    fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    fn small_circle() -> Circle {
        Circle { radius: 1.0 }
    }
}

fn main() {
    let mut year = String::new();
    println!("Enter the year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();

    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year", year);
    } else {
        println!("{} is not a leap year", year);
    }

    let circle1 = Circle { radius: 5.0 };
    println!("The diameter of the circle is: {}", circle1.diameter());
    println!(
        "The diameter of the small circle is: {}",
        Circle::small_circle().diameter()
    );

    let hoge = if year == 2021 { "2021" } else { "not 2021" };
    println!("The year is {}", hoge);

    let value = Some(100);
    let value2: i32 = 100;

    if let Some(s) = value {
        println!("The value is: {}", s);
    } else {
        println!("The value is None");
    }

    match value {
        Some(s) if s > 5 => println!("The value is greater than 5"),
        Some(v) => println!("The value is: {}", v),
        None => println!("The value is None"),
    }

    match value2 {
        1 | 2 => println!("The value is 1 or 2"),
        3..=10 => println!("The value is between 3 and 10"),
        _ => println!("The value is something else"),
    }

    let mut counter = 0;
    let res = loop {
        println!("The counter is: {}", counter);
        if counter == 10 {
            break;
        }
        counter += 1;
    };

    counter = 0;
    let res2 = while counter < 10 {
        println!("The counter is: {}", counter);
        counter += 1;
    };

    let mut counterOption = Some(0);
    while let Some(i) = counterOption {
        if i == 10 {
            counterOption = None;
        } else {
            println!("The counter is: {}", i);
            counterOption = Some(i + 1);
        }
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in v.iter() {
        println!("The value is: {}", i);
    }

    let mut one = 1;
    let plus_one = move |x| x + one;

    one += 1; // move つけないと、already borrowed エラーが出る
              // クロージャ作る時に、one をクロージャに貸しているので、one はクロージャに所有されている
              // そのため、one に対して変更を加えることができない
              // move をつけることで、クロージャに関数定義の外側にある変数の値がコピーされる
    println!("The value is: {}", plus_one(5));
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
