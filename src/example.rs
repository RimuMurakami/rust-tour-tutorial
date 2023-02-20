fn example() -> i32 {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        _ => "ホットドッグではありません",
    };
    println!("{}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("{v}");

    v + 4
}
