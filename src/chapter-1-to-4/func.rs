fn func() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{c}");

    let t = false;
    println!("{}", t as u8);

    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("{}", add(42, 13));
}
