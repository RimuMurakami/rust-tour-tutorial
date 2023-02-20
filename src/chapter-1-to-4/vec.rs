fn vec() {
    let mut vec = Vec::new();
    vec.push(1.45);

    let string_vec = vec![String::from("Hello"), String::from("world!")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}
