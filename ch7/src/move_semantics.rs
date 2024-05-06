#[cfg(test)]
mod tests {
    use crate::{Child, CopyableChild, CopyableParent, Parent};

    use super::*;

    #[test]
    fn test_move_semantics() {
        // Move Semantics
        let p1 = Parent(1, Child(11), Child(12));
        let p2 = p1; // 代入すると、p2 に所有権が移動される, p1 は値の所有権を失う（ムーブアウトする）

        // print!("p1: {:?}", p1); // p1 はムーブアウトされているので、ここでエラーする

        // pattern matching
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {
            // 文字列が見つかりました
            println!("found a string");
        }

        // println!("{:?}", s); // s はパターンマッチにより, _s にムーブアウトされているのでエ゙ラー

        // function call
        let s = String::from("Hello!");
        takes_ownership(s); // s は関数の引数にムーブアウトされる

        // println!("{:?}", s); // s は関数呼び出しにより、some_string にムーブアウトされているのでエラー

        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }

        // closure
        let mut p3 = 4;
        let closure = || {
            println!("Closure: {:?}", p3);
        };
        // p3 = 3; // p3 はクロージャによりムーブアウトされるので、ここでエラーする
        closure();

        // return
        let s = String::from("Hello!");
        let s2 = returns_ownership(s); // s は関数の戻り値にムーブアウトされる

        // println!("{:?}", s); // s は関数呼び出しにより、s2 にムーブアウトされているのでエラー

        fn returns_ownership(some_string: String) -> String {
            some_string
        }

        // Copy Semantics
        // Copy Trait を実装した型はムーブされず、コピーされるようになる
        // Copy Trait を実装する型は下記を満たす必要がある
        // - 自身が Clone トレイトを実装している
        // - 自身、および全てのフィールドの型が Copy トレイトを実装している
        // - 自身、および全てのフィールドの型が Drop トレイト（デストラクタ）を実装していない

        let cp1 = CopyableParent(1, CopyableChild(11), CopyableChild(12));
        let cp2 = cp1; // CopyableParent は Copy トレイトを実装しているので、ムーブではなくコピーされる

        println!("cp1: {:?}", cp1); // ok!

        // スカラ型：bool, char, i32, usize, i64...
        // 不変の参照型：&T, 生ポインタ型：*const T, *mut T（可変の参照型：&mut T は Copy トレイトを実装していない）
        // 関数ポインタ型：fn(T) -> U
        // タプル型：(T1, T2, T3, ...) ただし、全ての要素が Copy トレイトを実装している場合
        // 配列型：[T; N] ただし、全ての要素が Copy トレイトを実装している場合

        let a = 1;
        let b = a;
        println!("a: {}, b: {}", a, b); // ok!

        let t = (1, 2);
        let u = t;
        println!("t: {:?}, u: {:?}", t, u); // ok!

        //借用

        let s = String::from("Hello");
        fn takes_reference(s: &String) {
            println!("{}", s);
        }
        takes_reference(&s);
        println!("{}", s);
    }
}
