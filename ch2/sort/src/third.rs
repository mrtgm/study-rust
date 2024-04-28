use super::SortOrder;
use std::cmp::Ordering;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // Rust は if 式があるため、三項演算子は存在しない（代入できる）
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };

    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (comparator(&x[i], &x[mid_point + i])) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::SortOrder::*;
    use super::*;
    use crate::utils::*;
    // 構造体（関連する値を一つにまとめる） Student を定義
    // あとで Student同士を asset_eq で比較するので、PartialEq トレイトを実装　debug マクロで表示するために Debug トレイトを実装
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    // impl（実装）ブロック内に関連関数やメソッドを定義
    // new は「関連関数」（クラスメソッドに似た概念）
    impl Student {
        fn new(frist_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: frist_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    // impl PartialEq for Student {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.first_name == other.first_name
    //             && self.last_name == other.last_name
    //             && self.age == other.age
    //     }
    // }

    #[test]
    fn cmp() {
        let a = 1;
        let b = 2;
        println!("{:?}", a.cmp(&b));
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Greater);
    }

    #[test]
    fn sort_students_by_age_ascending() {
        let ash = Student::new("Ash", "Ketchum", 10);
        let misty = Student::new("Misty", "Waterflower", 11);
        let brock = Student::new("Brock", "Takeshi", 15);
        let pikachu = Student::new("Pikachu", "Pokemon", 3);

        let mut x = vec![&ash, &misty, &brock, &pikachu];
        let expected = vec![&pikachu, &ash, &misty, &brock];

        assert_eq!(
            sort_by(&mut x, &|a: &&Student,
                              b: &&Student|
             -> std::cmp::Ordering {
                a.age.cmp(&b.age)
            }),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let ash = Student::new("Ash", "Ketchum", 10);
        let misty = Student::new("Misty", "Waterflower", 10);
        let brock = Student::new("Brock", "Takeshi", 15);
        let pikachu = Student::new("Pikachu", "Pokemon", 3);

        let mut x = vec![&ash, &misty, &brock, &pikachu];
        let expected = vec![&ash, &brock, &misty, &pikachu];

        assert_eq!(
            sort_by(&mut x, &|a: &&Student,
                              b: &&Student|
             -> std::cmp::Ordering {
                a.first_name
                    .cmp(&b.first_name)
                    // 氏名が同じ場合は名前で比較
                    // Ordering::then_with は引数のないクロージャをとり、
                    // Equal ならその値を返す
                    // Equal でないなら、引数のクロージャを実行してその結果を返す
                    .then_with(|| a.last_name.cmp(&b.last_name))
            }),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            let mut x = refined_new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = refined_new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }
}

// クロージャは実装によって異なる型トレイトを自動的に実装する
// Fn トレイトはクロージャが引数を取ることを示す
// FnMut トレイトはクロージャが可変の参照を取ることを示す
// FnOnce トレイトはクロージャが所有権を取ることを示す
