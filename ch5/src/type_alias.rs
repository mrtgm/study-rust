// type alias
type UserName = String;
type Id = i64;
type Timestamp = i64;

// tuple-like struct
// 単純な型エイリアスだと、同じ構造の型を区別できず、変数の順番などを間違して指定してしまう可能性がある
// 一種の Branded Type として使う
struct UserNameTuple(String);
struct IdTuple(i64);
struct TimestampTuple(i64);

type User = (Id, UserName, Timestamp);

#[derive(Default)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

struct Vertex(i32, i32);
struct Triangle(Vertex, Vertex, Vertex);

// 参照つきの構造体、ライフタイム指定子を用いる
struct StrRefs<'a> {
    s1: &'a str,
    s2: &'a str,
}

// 列挙型（代数的データ型）
// enum は複数の型をまとめた型を定義する
#[derive(Debug, PartialEq)] // PartialEq は比較演算子を使えるようにする, Debug はデバッグ出力({:?})できるようにする
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

// Open, AssignedTo, Working, Done のいずれかなタスク　それぞれにデータを持たせる
#[derive(Debug)]
pub enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

use crate::test;
use Task::*;

fn type_alias() {
    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    let user = new_user("mika".to_string(), id, now);

    let triangle = Triangle(Vertex(0, 0), Vertex(3, 0), Vertex(2, 2));
    let polygon = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        stroke_width: 1,
        fill: (0, 0, 255),
    };

    let Polygon { fill: hoge, .. } = polygon;
    assert_eq!(hoge, (0, 0, 255));

    assert_eq!(user.1, "mika");
    assert_eq!(polygon.vertexes[0], (0, 0));

    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        ..Default::default()
    };

    assert_eq!(polygon2.stroke_width, 0);

    fn say_something(weekday: Weekday) {
        if weekday == Weekday::Friday {
            println!("TGIF!");
        } else {
            println!("まだ{:?}か", weekday);
        }
    }

    say_something(Weekday::Friday);

    // 代数的データ型とパターンマッチによる処理、つよい
    let tasks = vec![
        Open,
        AssignedTo("junko".to_string()),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 19,
        },
        Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            Open => println!("Task {} is open", i),
            AssignedTo(assignee) => println!("Task {} is assigned to {}", i, assignee),
            Working {
                assignee,
                remaining_hours,
            } => {
                println!(
                    "Task {} is being worked on by {} with {} hours remaining",
                    i, assignee, remaining_hours
                )
            }
            Done => println!("Task {} is done", i),
        }
    }

    // キャスト
    let i1 = 42; //i32
    let f1 = i1 as f64; //i32 -> f64

    // タプル, 配列のキャスト
    let t1 = (1, 'a');
    let a1 = [1, 2, 3];
    let v1 = vec![1, 2, 3];

    let t2 = (t1.0 as i64, t1.1 as u32);
    let a2: [i64; 3] = [1, 2, 3];
    let v2: Vec<i64> = v1.iter().map(|&x| x as i64).collect();

    // 複合型のキャスト
    let v4: Vec<u8> = From::from("hello");

    // 型強制
    let v5: Vec<u8> = vec![65, 66, 67]; // i32 -> u8 への型強制
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_alias() {
        type_alias();
    }
}
