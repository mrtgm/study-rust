use std::{time, thread};

fn main() {
    let n1 = 1200;
    let n2 = 1000;

    // spawn で子スレッドを立ち上げ、親スレッドと並行して実行する
    // thread::spawn はスレッドのハンドルを返す
    let child = thread::spawn(move || {
        heavy_work("child", n1);
    });

    let s1 = heavy_work("main", n2);

    // join で子スレッドの終了を待つ
    // ハンドルに対して join を呼び出す

    match child.join() {
        Ok(s2) => println!("sum: {:?}{:?}", s1, s2),
        Err(e) => eprintln!("error: {:?}", e),
    }
}

fn heavy_work(name: &str, n: u64) -> u64 {
    println!("{}: started", name);
    thread::sleep(time::Duration::from_millis(n));
    let sum = (1..=n).sum();
    println!("{}: ended", name);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heavy_work() {
        assert_eq!(heavy_work("test", 100), 5050);
    }

    #[test]
    fn test_main() {
        main();
    }
}
