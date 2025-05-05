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
        }
    }
    println!("Sproing! What was that?");
}
