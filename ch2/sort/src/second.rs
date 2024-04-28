use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

// pub はこのモジュールが外部に公開されることを示す
// 32ビット符号なし整数のスライス（一次元配列）を引数に取り、ソートする関数 sort を定義
fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    //&mut は値をポインタ経由で「可変」「借用」することを示す、コピーせず代入して使用できる
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up)
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // この比較でコケるので、T が Ord トレイトを実装していることを明示
        // Ord は比較演算子を使える型に実装されるトレイト
        // PartialOrd とは異なり、全順序関係を持つ（全ての値が比較可能） PartialOrd は一部の値（NaN）が比較不可能
        // u32 型は Ord / PartialOrd トレイトを実装してるが、f32 型は PartialOrd トレイトのみ実装している
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

// cargo test 実行時のみコンパイルされるテストコード
#[cfg(test)]
mod tests {
    // tests モジュール内で親モジュールの sort 関数を使用
    use super::sort;
    use crate::SortOrder::*;

    // test case には #[test] アトリビュートを付与
    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x: Vec<&str> = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];

        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x: Vec<&str> = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];

        assert_eq!(sort(&mut x, &Descending), Ok(()));

        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x: Vec<u32> = vec![10, 30, 11];
        let expected = sort(&mut x, &Ascending);

        assert!(expected.is_err());
    }
}
