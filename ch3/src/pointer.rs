pub fn pointer() {
    // 参照型
    fn f1(mut n: u32) {
        n = 1;
        println!("f1: n: {}", n);
    }

    // 関数の中で（所有権を移動させずに）値を代入/参照したい場合、参照を使ってポインタを渡す
    // 参照とはメモリ安全なポインタ
    // 参照は & 演算子を使って作成、参照外し（*）で値を取得
    // 参照を受ける関数は、引数でも型として参照型（&型 or &mut型）を指定する必要がある
    // & は、参照を作成する演算子でもあり、参照型を指す記号でもある
    // 引数に & 型を指定する場合、後置？
    fn f2(n_ptr: &mut u32) {
        println!("f2: n_prev: {:p}", n_ptr); // pointer address を表示

        // dereference: 参照外し、ポインタが指す値を取得する
        *n_ptr = 2;
        println!("f2: n_new: {}", *n_ptr);
    }

    let mut n = 0;
    println!("n: {}", n);

    f1(n);
    println!("n: {}", n);

    // &mut n で、n の値を指す可変の参照を作成
    f2(&mut n);
    println!("n: {}", n);
    println!("n: {:p}", &n);

    let c1 = 'A';
    let c1_ptr: *const char = &c1; //不変の生ポインタ
    assert_eq!(unsafe { *c1_ptr }, 'A'); //生ポインタを参照外し、これはunsafe

    fn double(n: i32) -> i32 {
        n * 2
    }

    fn abs(n: i32) -> i32 {
        if n >= 0 {
            n
        } else {
            -n
        }
    }

    let mut f: fn(i32) -> i32 = double; // 関数ポインタ（fn pointer type）
                                        // 型注釈をつけないと、関数定義型（fn item type）として推論され、関数ごとに違う型になるので、後続する代入に失敗する
    assert_eq!(f(-42), -84);

    f = abs;
    assert_eq!(f(-42), 42);

    assert_eq!(
        std::mem::size_of::<fn(i32) -> i32>(),
        std::mem::size_of::<usize>()
    ); // 関数ポインタはusize型と同じサイズ

    // クロージャは内部で引数として受け取った or スコープ外の変数を参照、変更することができる　これをキャプチャという
    let mut n = 0;
    let mut c = || {
        n += 1;
        n
    };

    assert_eq!(c(), 1);
    assert_eq!(c(), 2);

    // 何もキャプチャしないクロージャは関数ポインタとしても扱える
    let f: fn() -> i32 = || 42;
    assert_eq!(f(), 42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer() {
        pointer();
    }
}
