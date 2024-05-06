use std::hash::DefaultHasher;

pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

pub struct Iter<'a, T> {
    elements: &'a [T], // 構造体や列挙型で参照型のフィールドを持つ場合、ライフタイムパラメータを指定する必要がある
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T; //関連型？？

    fn next(&mut self) -> Option<Self::Item> {
        // Option のライフタイムと、Iter のライフタイムを一致させる
        if self.pos == self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default) //なぜここで T 型に推論されるのかわからん、トレイト境界まわりの話？
            .take(size) //　任意個数の T 型の要素を生成するイテレータを生成
            .collect::<Vec<_>>() // ここで _ は T になる、Vec<T> に詰める
            .into_boxed_slice() // Box<[T]> にして固定長のメモリを確保
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        self.elements[self.len] = element; // ここで self.elements は Box<[T]> なのでインデックスアクセスできる
                                           // Box<[T]> に所有権が移動する
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index]) //self が持つ不変の参照を返す
        } else {
            None
        }
    }

    // elementsを拡張する（より大きなサイズで作り直す）
    fn grow(&mut self) {
        if self.capacity() == 0 {
            // 現在のelementsが空なら
            // 1要素分の領域を確保する
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elementsを置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }

            // for (i, elem) in old_elements.into_iter().enumerate() {
            //     // iter は不変の参照を返すので、所有権をムーブすることはできない
            //     self.elements[i] = elem;
            // }
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        match self.get(index) {
            Some(v) => v,
            None => &default,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // 借用に束縛された変数経由で、所有権をムーブすることはできない
            // let elem = self.elements[self.len]; self.elements は借用してるので、所有権を勝手に移動できない
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            // 値を奪うことはできないが、別の値に置き換えることでムーブすることが可能
            // Default::default() は T 型のデフォルト値を返し、replace は元の値を返しつつ新しい値を代入する
            Some(elem)
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}
