use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        // sample の戻り地の型は v の型から自動的に決まる（この場合は u32）
        // &Standard は標準の一様分布を表す
        v.push(rng.sample(&Standard));
    }

    v
}

// イテレータとコレクタ使って書き直す
pub fn refined_new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // sample_iter は無限に乱数を生成するイテレータを作る
    // take は最初の n 要素だけを取り出す
    // collect はイテレータからベクタを作る
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // イテレータを使い、隣り合う要素を比較していく
    // windows(2) はスライスから2要素づつ取り出し、新しいイテレータを作る
    // all はイテレータの全ての要素が条件を満たすかどうかを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
