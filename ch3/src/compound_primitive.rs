pub fn compound_primitive() {
    // tuple
    let t = (1, 2, 3);
    let (a, b, c) = t;
    assert_eq!(a, 1);
    assert_eq!(t.1, 2);

    let mut t2 = (88, true);
    t2.0 += 100;
    assert_eq!(t2.0, 188);

    let t3 = ((1, 2), (3, 4)); // パターンマッチで分解
    let ((a, b), _) = t3;

    // 書き換え時
    let mut t4 = ((1, 2), (3, 4));
    let ((ref mut a, ref mut b), _) = t4;
    *a = 10;

    // array
    let a = [1, 2, 3]; // 配列はすべて同じ型の要素を持つ　コンパイル時に要素数が決まる
    let mut a2 = [0; 100]; // 100要素の配列、全要素が0で初期化 Copy トレイトを実装している型のみ初期化可能

    a2[0] = 1;
    let index = 10;
    a2[index] = 10; // index が範囲外の場合、パニックする
    a2.get(index); // Option(Some or None) を返す

    // a2.push(1); // 配列は固定長なので、要素を追加できない

    for ch in a2.iter() {
        // イテレータを使って要素にアクセス
        println!("{}", ch);
    }
    for ch in a2.iter_mut() {
        *ch += 1; // イミュータブルな参照を使って書き換え
    }

    // slice
    // スライスとは、、？ 配列を参照して、その一部を参照・操作する
    fn print_info(name: &str, sl: &[char]) {
        println!(
            "{:9} - {}, {:?}, {:?}, {:?}",
            name,
            sl.len(),
            sl.first(),
            sl[1],
            sl.last()
        );
    }

    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);

    print_info("&a1[..]", &a1[..]); // 全要素のスライス
    print_info("&a1", &a1); // 同上
    print_info("&a1[1..3]", &a1[1..3]); // 2番目から3番目（一つ手前）までのスライス　start <= n < end

    // mutable slice
    let mut a2 = [0, 1, 2, 3, 4];
    let s1 = &mut a2[1..3];
    s1[0] = 10;
    // methods
    s1.swap(0, 1);
    s1.contains(&10);
    // mutable なスライスだけで可能な破壊的操作
    s1.sort();
    // s1.split_at_mut(3);
    // s1.reverse();

    // 文字列スライス(str), char は１文字のみ
    let s = "Hello, Rust!"; //&str型
    let s2 = "
        Hello,
        Rust!
    ";
    let sr = r"Hello, Rust!\\\\####"; // raw string
    let mut lines = "foo\nbar\nbaz".lines(); //イテレータを返す
    let foo_line = lines.next();
    assert_eq!(foo_line, Some("foo"));

    if let Some(foo) = foo_line {
        println!("{}", foo);
        assert!(foo.contains("foo"));
        assert!(foo.find("a").is_none());
    }

    lines.map(str::trim);

    // str の長さ, utf8 のバイト数
    let s1 = "a";
    let s2 = "あ";
    let s3 = "🦀";
    let s4 = "🇯🇵";
    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 3);
    assert_eq!(s3.len(), 4);
    assert_eq!(s4.len(), 8);

    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('H'));

    // 文字列の書き換えには String 型を使う
    let mut s = String::from("Hello, ");
    s.push('R');
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compound_primitive() {
        compound_primitive();
    }
}
