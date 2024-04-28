fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
        //逆ポーランド記法　
        // exp 変数を、RPN 形式の文字列に「束縛」する　束縛は関数型言語の用語
    let ans = rpn(exp);

    debug_assert_eq!("26.2840", format!("{:.4}", ans)); // デバッグビルド時のみ有効
    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64 {
    //f64型（浮動小数点数）を返す
    let mut stack = Vec::new();
        // 空のスタックを用意する
        // stack はミュータブル、値の変更を許す

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            // token が f64 型の数値ならスタックに積む
            stack.push(num);
        } else {
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                    //&は、スタックが「束縛」されたベクタへの参照を示す
                    //mutは、ミュータブル（変更可能）な参照を示す
                    //|x, y| x + y はクロージャ（無名関数）で、引数 x, y を取り x + y を返す
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                _ => panic!("Unknown operator: {}", token),
            }
        }
    }

    stack.pop().expect("Stack underflow.")
        // スタックから計算結果を取り出す, なければエラー
        // セミコロンがない、式の最後の値が返り値になる
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
// fn 関数名<F>(..., 引数名: F) という形式で、ジェネリック関数を定義
// where F: Fn(引数の型, 引数の型) -> 戻り値の型 という形式で、ジェネリック関数の制約を定義
// where 節以下をトレイト境界と呼ぶ
// ジェネリクス関数が受け付ける型の境界を指定するもの、他言語の T extends X ...と同じ
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x,y);
        stack.push(z);
    } else {
        panic!("Stack underflow.");
    }
}
