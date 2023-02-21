fn text() -> Result<(), std::num::ParseIntError> {
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len());
    for char in &chars {
        print!("{char}");
    }
    println!();
    println!("{}", chars[3] as u32);

    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);

    let a = 34;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}
