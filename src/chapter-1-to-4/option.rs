struct BagOfHolding<T> {
    item: Option<T>,
}

fn option() {
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("This is None");
    } else {
        println!("something");
    }

    let i32_bag = BagOfHolding { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("something");
    } else {
        println!("None");
    }

    match i32_bag.item {
        Some(v) => println!("something {v}"),
        None => println!("nothing"),
    }
}
