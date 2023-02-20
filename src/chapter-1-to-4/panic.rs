fn do_some(bool: bool) -> Result<bool, String> {
    if bool {
        Ok(true)
    } else {
        Err(String::from("Falseです"))
    }
}

fn panic() -> Result<(), String> {
    // let result = do_some(true);

    // match result {
    //     Ok(v) => println!("発見 {}", v),
    //     Err(_e) => {
    //         return Err(String::from("mainでエラー発生"));
    //     }
    // }

    let v = do_some(true)?;
    println!("発見 {}", v);

    // panic!!
    let v = do_some(false).unwrap();
    println!("発見 {}", v);

    Ok(())
}
