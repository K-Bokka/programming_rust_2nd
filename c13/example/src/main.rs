use rand::random;

fn main() {
    println!("Chapter 13");

    // 13.1 Drop
    #[derive(Debug)]
    struct Application {
        name: String,
        nickname: Vec<String>,
    }

    impl Drop for Application {
        fn drop(&mut self) {
            println!("Dropping {}", self.name);
            if !self.nickname.is_empty() {
                println!("AKA {}", self.nickname.join(", "));
            }
            println!();
        }
    }

    {
        let mut a = Application {
            name: "Zero".to_string(),
            nickname: vec!["cloud collector".to_string(), "king of gods".to_string()],
        };
        println!("a is {:?}", a);
        println!("before assignment");
        a = Application {
            name: "Hera".to_string(),
            nickname: vec![],
        };
        println!("a is {:?}", a);
        println!("at end of block");
    }

    fn complicated_condition() -> bool {
        random::<bool>()
    }

    let p;
    {
        let q = Application {
            name: "Athena".to_string(),
            nickname: vec!["queen of heaven".to_string(), "queen of men".to_string()],
        };
        if complicated_condition() {
            p = q;
            println!("p is {:?}", p);
        }
    }
    println!("Sproing! What was that?");

    // 13.2 Sized
    // Sizedかも
    #[allow(dead_code)]
    struct S<T: ?Sized> {
        b: Box<T>,
    }

    #[derive(Debug)]
    struct RcBox<T: ?Sized> {
        #[allow(dead_code)]
        ref_count: usize,
        value: T,
    }

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };

    use std::fmt::Display;
    #[allow(unused_variables)]
    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;

    fn display(boxed: &RcBox<dyn Display>) {
        println!("For your enjoyment: {}", &boxed.value);
    }

    display(&boxed_lunch);
    
    // 13.3 Clone
    // 13.4 Copy
}
