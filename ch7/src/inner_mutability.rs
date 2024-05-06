use std::cell::RefCell;
use std::collections::HashSet;

struct B {
    c: char,
    s: RefCell<String>,
}

// TLS (Thread Local Storage) はスレッドごとに独立したメモリ領域を持つ、スレッドごとに状態共有したいとき
thread_local! {
    static RABBITS: RefCell<HashSet<&'static str>> = {
        let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
        RefCell::new(rb)
    }
}

pub fn inner_mutability() {
    let mut b = B {
        c: 'a',
        s: RefCell::new("alex".to_string()),
        // コンパイル時の借用チェックを迂回し、不変の借用経由でもデータを変更できるようにする
        // struct のフィールドを mutable にしたいときはこれ、デフォは immutable
    };

    let rb = &b;
    rb.s.borrow_mut().push_str("b");

    {
        let rbs = b.s.borrow();

        // let mut rbs = b.s.borrow_mut(); // すでに不変の参照を作ってる場合、可変の参照を作ることはできない（パニック=実行時エラーする）

        let mut rbs = b.s.try_borrow_mut(); // try_borrow_mut は失敗すると Result 型でエラーを返す
    }

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    std::thread::spawn(|| {
        RABBITS.with(|rb| {
            rb.borrow_mut().insert("ドワーフホト");
        })
    })
    .join()
    .expect("Thread error");

    fn apply_fn<F>(f: &F, ch: char)
    where
        F: Fn(char) -> bool,
    {
        assert!(f(ch));
    }

    fn apply_fn_mut<F>(f: &mut F, ch: char)
    where
        F: FnMut(char) -> bool,
    {
        assert!(f(ch));
    }

    fn apply_fn_once<F>(f: F, ch: char)
    where
        F: FnOnce(char) -> bool,
    {
        assert!(f(ch));
    }

    let s1 = "read-only".to_string();
    let s3 = "consumable".to_string();

    // is_alphabetic(s1);

    let mut lookup = || s1.find('a').is_some(); // Fn  s1 は不変の参照を持つ

    println!("{:?}", s1);
    let mut consume = || {
        let bytes = s3.into_bytes();
        bytes.contains(&('a' as u8))
    };

    fn is_alphabetic(str: &str) -> bool {
        str.is_ascii() && str.chars().all(|c| c.is_alphabetic())
    }
    // apply_fn(&lookup, 'r');
    // apply_fn_mut(&mut lookup, 'o');

    // apply_fn_once(consume, 'y');

    let handle = std::thread::spawn(lookup);

    // println!("{:?}", &s3);
}
