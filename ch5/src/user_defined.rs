use std::collections::{BTreeMap, HashMap};

fn user_defined() {
    // # Box
    let t1 = (3, "birds".to_string());
    let mut b1 = Box::new(t1); //tuple がヒープに移動
    (*b1).0 += 1;

    // println!("{:?}", t1); // ヒープに移動したのでエラーになる
    // String は
    //  1. ヒープ領域にあるデータへのポインタ（usize）
    //  2. データの長さ (usize)
    //  3. データの最大容量 (usize)
    // の3つの要素からなる構造体、全部で24バイト
    // t1 の宣言時、ヒープ領域にデータが確保され、そのアドレスが t1 に格納される
    // Box::new(t1) でヒープ領域に String を含む tuple 全体のデータがコピーされ、そのアドレスが b1 に格納される
    // 関数の実行が終わるとスタック領域が解放されるため、ヒープ領域に移動したデータも解放される

    // コンパイル時にデータサイズが決まらない型を扱うとき（再帰的？なデータ構造？）をやるとき？？に使う
    // でかいデータをコピーせず、所有権だけ他人に移したいとき
    // トレイトオブジェクトを使うとき

    // # Vec
    let v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.0, 0.5];

    // let v3 = vec![v1, v2]; // エラー、同じ型じゃないとダメ
    let v3 = vec![vec!['a', 'b'], vec!['c']]; // 入れ子

    let mut v4 = vec![0; 10]; // 0 が 10 個
    v4.push(11); // 後から追加できる
    let e = v4.pop(); // 末尾を取り出す
    v4.iter().for_each(|x| print!("{} ", x)); // iter

    println!("{:?}", e);
    println!("{:?}", v4.len());

    // Vec は要素数が動的に増えるので、あらかじめ余分なメモリをヒープに確保する
    // Box は動的に追加できず、固定サイズのデータをヒープに確保する

    let mut v1 = vec![1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity()); // 4, 6

    let s1 = v1.into_boxed_slice();
    let v2 = s1.into_vec();
    println!("v2 len: {}, capacity: {}", v2.len(), v2.capacity()); // 4, 4

    // # HashMap, BTreeMap
    let mut m1 = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.get("b"), Some(&3));
    let d = m1.entry("d").or_insert(0); // d がなければ 0 を挿入
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));

    // # String
    let mut s1 = "ラズベリー".to_string(); //strリテラルからStringを作成

    s1.push_str("タルト");
    s1.push('と');
    let s2 = "チョコレート".to_string();
    s1.push_str(&s2); // & つけると &str に変換
    assert_eq!(s1, "ラズベリータルトとチョコレート");

    let s3 = "34".to_string();
    let n1: Result<i32, _> = s3.parse(); // Result<i32, ParseIntError>
    assert_eq!(n1, Ok(34));

    // utf16 から utf8 への変換
    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];
    if let Ok(s) = String::from_utf16(&utf16) {
        assert_eq!(s, "ab漢字");
    }

    // 範囲
    let a = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[..3], ['a', 'b', 'c']);
    assert_eq!(a[..=3], ['a', 'b', 'c', 'd']);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c']);

    // # Option
    // パターンマッチで取り出す
    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }
    if let Some(s) = o1 {
        assert_eq!(s, 10);
    }
    let mut o2 = None;
    o2.unwrap_or_else(|| "o2 is None");

    // map, and_then, or_else で、Option の中身をコンビネータで変換
    // コンビネータとは、関数型プログラミングで使われる、関数を引数に取る関数
    let mut o3 = Some(10);
    let mut o4 = o3.map(|s| s + 5).and_then(|s| Some(s + 3));

    fn add_elems(s: &[i32]) -> Option<i32> {
        let s0 = s.get(0)?; // get は Option<&T> を返す、? は Some なら中身を取り出す、None なら関数から抜けて None を返す
        let s3 = s.get(3)?;
        Some(s0 + s3)
    }

    let a = [1, 2, 3];
    assert_eq!(add_elems(&a), None);
    let b = [1, 2, 3, 4, 5];
    assert_eq!(add_elems(&b), Some(5));

    // # Result
    // Rust では、例外（Exception）ではなく、Result を使ってエラーを扱う
    // Result は Ok か Err のどちらかを返し、Err にメッセージを含めることで、エラーの理由を伝えられる
    assert_eq!("10".parse::<i32>(), Ok(10));

    // map_err でエラーメッセージを変換
    fn add1(s0: &str, s1: &str) -> Result<i32, String> {
        let s0 = s0
            .parse::<i32>()
            .map_err(|_| "s0 is not a number".to_string())?;
        let s1 = s1
            .parse::<i32>()
            .map_err(|_| "s1 is not a number".to_string())?;
        Ok(s0 + s1)
    }

    Ok::<(), ()>(()).ok(); // Option に変換
    Some(()).ok_or_else(|| 0); // Result に変換

    assert_eq!(add1("3", "127"), Ok(130));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_defined() {
        user_defined();
    }
}
