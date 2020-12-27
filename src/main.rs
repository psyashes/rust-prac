use std::thread;

// define struct
struct Brothers {
    big_bro: String,
    little_bro: String
}

// define implement
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

// Trait
trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uoooooohnnnnnnnn!!!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

struct Color {
    r: i32,
    g: i32,
}

// main function
fn main() {
    // construct struct
    let b = Brothers {
        big_bro: String::from("Mario"),
        little_bro: String::from("Luigi")
    };
    b.greet_by_big().greet_by_little();

    // vector
    let v = vec![1, 2, 3];
    for element in &v {
        println!("{}", element);
    }

    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};
    
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    // ownership
    let a = Color{r:255, g:255};
    println!("a content is {}", a.r);
    let b = a;
    println!("b content is {}", b.g);

    // borrow
    let mut x = 5;
    {
       let y = &mut x;
       dbg!(y);
    }
    {
        let y = &x;
        dbg!(y);
    }

    // thread
    let mut handles = Vec::new();
    for x in 1..10 {
        handles.push(thread::spawn(move || {
            println!("Sheep :{}", x);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }

    // panic
    panic!("In the end.");
}

