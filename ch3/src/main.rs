// Scaler
// - primitive
//   bool, char, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 ...
// Compound
//  - primitive
//    - tuple, array, slice, struct, enum, union...
//  - user defined
//    - struct, string, enum, union, Vec, HashMap, BTreeMap, HashSet, BTreeSet...

// アクセスできる内部構造を持つ型を Compound と呼ぶ

mod compound_primitive;
mod pointer;

use std::str::FromStr;

fn main() {
    // 整数・文字列リテラル
    let n = 10;
    let c = 'A';

    // ユニット型: 空を表す型、0バイトのサイズを持つ
    let u = ();

    fn hello() {
        println!("Hello");
    }

    assert_eq!(hello(), ()); // Void な関数の戻り値はユニット型
    assert_eq!(std::mem::size_of::<()>(), 0);

    // boolean
    let b = true;
    let b1 = !b;
    assert_eq!(std::mem::size_of::<bool>(), 1); //サイズは1バイト

    // 固定精度の整数

    // 1)ビット幅を指定するパターン
    let n1 = 10; // i32
    let n2 = 10u32; // u32 値の範囲：0~2^32-1（4,294,967,295）
    let n2i = 10i32; // i32 値の範囲：-2^31~2^31-1（-2,147,483,648~2,147,483,647）
    let n3 = 10u64; // u64 値の範囲：0~2^64-1（18,446,744,073,709,551,615）
    let n3i = 10i64; // i64　値の範囲：-2^63~2^63-1（-9,223,372,036,854,775,808~9,223,372,036,854,775,807）

    // 符号付き：i8, i16, i32, i64（負の値を含む）
    // 符号なし：u8, u16, u32, u64（負の値を含まない）

    assert_eq!(std::mem::size_of_val(&n1), 4);
    assert_eq!(std::mem::size_of_val(&n2), 4);
    assert_eq!(std::mem::size_of_val(&n2i), 4);
    assert_eq!(std::mem::size_of_val(&n3), 8);
    assert_eq!(std::mem::size_of_val(&n3i), 8);

    // 2)ターゲットのアーキテクチャに依存するパターン
    let n4 = isize::MAX; // isize: CPUのメモリアドレスのビット幅に依存する符号付き整数型
    let n5 = usize::MAX; // usize: CPUのメモリアドレスのビット幅に依存する符号なし整数型

    println!("isize: {}", n4); // 64bit符号付き整数の最大値、9,223,372,036,854,775,807
    println!("usize: {}", n5);
    assert_eq!(std::mem::size_of_val(&n4), 8);
    assert_eq!(std::mem::size_of_val(&n5), 8);

    // メソッド
    let s = "hello".to_string();
    let s1 = s.to_uppercase();

    let n6 = 10i32.pow(2);
    let n7 = 10f32.sqrt();
    let n8 = u32::from_str("10.5");

    // 桁溢れ
    let n9 = 10u8;
    let n10 = n9.wrapping_add(250); // 10 + 250 = 260 -> 4
    let n11 = n9.overflowing_add(250); // (10 + 250, true) -> (4, true)
    println!("n10: {}", n10); // 4
    println!("n11: {:?}", n11); // (4, true)

    // 浮動小数点数
    // f32: 単精度浮動小数点数
    // f64: 倍精度浮動小数点数

    let f1 = 10.0; // f64
    let f2 = 10.0f32; // f32

    assert_eq!(std::mem::size_of_val(&f1), 8);
    assert_eq!(std::mem::size_of_val(&f2), 4);

    // 丸め誤差
    let f3 = 0.1 + 0.2;
    println!("f3: {}", f3); // 0.30000000000000004

    let f4 = 1.0 / 10.0;
    println!("f3: {}", f4); // 0.1

    // 文字
    let c1 = 'A'; // char型、シングルクォート
    let c2 = 'あ'; // 日本語の文字も1文字として扱う
    let c3: char = '😦'; // 絵文字
                         // '🇯🇵'; 合字はコンパイルエラー
    assert_eq!(std::mem::size_of_val(&c1), 4); // 4バイト
}
