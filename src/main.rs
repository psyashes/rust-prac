struct Brothers {
    big_bro: String,
    little_bro: String
}

impl Brothers {
    fn greet_by_big(&self) -> &Self {
        println!("I am {}.", self.big_bro);
        self
    }

    fn greet_by_little(&self) -> &Self {
        println!("I am {}.", self.little_bro);
        self
    }
}

fn main() {
    let b = Brothers {
        big_bro: String::from("Mario"),
        little_bro: String::from("Luigi")
    };

    b.greet_by_big().greet_by_little();
}
