// #![allow(dead_code)]

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

pub fn animal2() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claw, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                };
                println!(
                    "ferris is a crab with {} {} claws.",
                    num_claw, size_description
                );
            }
            _ => println!("ferris is a crab with some other weapon."),
        },
        _ => println!("ferris is some other animal."),
    }
}
