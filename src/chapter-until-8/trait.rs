struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

// fn generic_make_noise<T>(creature: &T)
// where
//     T: NoiseMaker,
// {
//     // we know the real type at compile-time
//     creature.make_noise();
// }

fn generic_make_noise(creature: &impl NoiseMaker) {
    creature.make_noise();
}

fn tr() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    generic_make_noise(&creature);
}
