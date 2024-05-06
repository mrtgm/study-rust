use std::collections::HashMap;

fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    match map.get_mut(&key) {
        // get_mut は map に対する可変の参照を返す（&mut String）。
        // Some(value) により、value がその参照に束縛される。したがって、value が生存している間は map への借用も生存している。
        // 従来のコンパイラは map.insert で map に対する可変の参照を取得しようとすると、借用規則への違反とみなしてエラーを出していた。
        // 可変の参照を同時に複数持つことはできないため。
        // しかし、NLL は制御構造を適切に解釈し、map.insert が実行される段階では Some(value)による借用は終了しており、 map に対する可変の参照が束縛されていないことを理解する。
        Some(value) => println!("Processing key: {}, value: {}", key, value),
        None => {
            println!("Key {} not found, inserting default value", key);
            map.insert(key, String::from("default"));
        }
    }
}
