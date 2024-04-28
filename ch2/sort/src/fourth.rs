use super::SortOrder;
use rayon;
use std::cmp::Ordering;

pub fn sort<T: Send + Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

const PARALLEL_THRESHOLD: usize = 4096;

// 並行処理を行う際、スレッド間で共有することができるデータの型には Send トレイトを実装する必要がある
// また、共有されるクロージャは Sync トレイトを実装している必要がある
fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        if mid_point > PARALLEL_THRESHOLD {
            // 所有権の問題、ある値に対する可変の参照を同時に複数持つことはできない
            // x[..mid_point] と x[mid_point..] に対してそれぞれ可変の参照を持つことはできない
            // ので、スライスをコピーする
            // rayon::join(
            //     || do_sort(&mut x[..mid_point], true, comparator),
            //     || do_sort(&mut x[mid_point..], false, comparator),
            // );

            // このように書き換える
            let (first, second) = x.split_at_mut(mid_point);
            rayon::join(
                || do_sort(first, true, comparator),
                || do_sort(second, false, comparator),
            );
        } else {
            do_sort(&mut x[..mid_point], true, comparator);
            do_sort(&mut x[mid_point..], false, comparator);
        }
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);

        if mid_point > PARALLEL_THRESHOLD {
            rayon::join(
                || sub_sort(first, forward, comparator),
                || sub_sort(second, forward, comparator),
            );
        } else {
            sub_sort(&mut x[..mid_point], forward, comparator);
            sub_sort(&mut x[mid_point..], forward, comparator);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
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
    T: Send,
    F: Sync + Fn(&T, &T) -> Ordering,
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
