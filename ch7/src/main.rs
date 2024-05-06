use std::ops::Drop;
use std::rc::Rc;

#[derive(Debug)]
pub struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
pub struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CopyableParent(usize, CopyableChild, CopyableChild);

#[derive(Copy, Clone, Debug)]
pub struct CopyableChild(usize);

mod move_semantics;
mod nll;

mod inner_mutability;
mod toyvec;

use toyvec::main::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));

    let hoge = "Hoge".to_string();

    v.get_or(1, &hoge);

    let item = v.iter();

    // vec -> iter -> for
    let i = "Hello".to_string().chars().into_iter();

    //生ポインタをベースにした、参照カウントを持つポインタ
    let mut rc1 = Rc::new(Child(1));

    println!("(a) count {}, rc1: {:?}", Rc::strong_count(&rc1), &rc1);

    {
        let rc2 = Rc::clone(&rc1);
        println!(
            "(b) count {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1),
            &rc1,
            &rc2
        );
    }
    println!("(c) count {}, rc1: {:?}", Rc::strong_count(&rc1), &rc1);

    if let Some(e) = Rc::get_mut(&mut rc1) {
        //参照カウントが1の場合のみ、可変参照を取得できる
        e.0 += 1;
    }

    println!("(d) count {}, rc1: {:?}", Rc::strong_count(&rc1), &rc1);

    let weak = Rc::downgrade(&rc1); // 共同所有権を持たない Weak ポインタを作成
    println!("(e) count {}, rc1: {:?}", Rc::strong_count(&rc1), &rc1);

    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            &rc1,
            &rc3
        );
    }

    std::mem::drop(rc1);
    println!("(g) count 0, rc1: {:?}", weak.upgrade());

    // 循環参照（複数のポインタが互いに参照し合うことで解放不能になる状態）を避ける

    inner_mutability::inner_mutability();
}
