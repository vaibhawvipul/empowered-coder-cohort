pub mod animal_traits {
    pub trait Animal {
        const SOUND: &'static str;

        fn create(name: String) -> Self;

        fn make_sound(&self);

        fn eat(&self) {
            println!("{} is eating.", self.name());
        }

        fn name(&self) -> &String;
    }

    pub struct Dog {
        name: String,
    }

    impl Animal for Dog {
        const SOUND: &'static str = "Woof!";

        fn create(name: String) -> Dog {
            Dog { name }
        }

        fn make_sound(&self) {
            println!("{} says {}", self.name, Self::SOUND);
        }

        fn name(&self) -> &String {
            &self.name
        }
    }

    pub struct Cat {
        name: String,
    }

    impl Animal for Cat {
        const SOUND: &'static str = "Meow!";

        fn create(name: String) -> Cat {
            Cat { name }
        }

        fn make_sound(&self) {
            println!("{} says {}", self.name, Self::SOUND);
        }

        fn eat(&self) {
            println!("{} is eating loudly.", self.name());
        }

        fn name(&self) -> &String {
            &self.name
        }
    }
}