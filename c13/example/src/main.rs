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
    // 13.5 Deref & DrefMut
    struct Selector<T> {
        elements: Vec<T>,
        current: usize,
    }
    use std::ops::{Deref, DerefMut};
    impl<T> Deref for Selector<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.elements[self.current]
        }
    }
    impl<T> DerefMut for Selector<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.elements[self.current]
        }
    }

    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };
    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'a';
    assert_eq!(s.elements, vec!['x', 'y', 'a']);

    let s = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 2,
    };
    fn show_it(thing: &str) {
        println!("{}", thing);
    }
    show_it(&s);

    fn show_it_generic<T: Display>(thing: T) {
        println!("{}", thing);
    }
    // show_it_generic(&s); // error[E0277]: `Selector<&str>` doesn't implement `std::fmt::Display`
    show_it_generic(&s as &str);
    show_it_generic(&*s);

    // 13.6 Default
    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<u32>, HashSet<u32>) =
        squares.iter().partition(|&x| x & (x - 1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");

    // 13.7 AsRef AsMut
    let dot_zshrc = std::fs::File::open("/Users/ak_yama/.zshrc");
    println!("{:?}", dot_zshrc);

    // 13.8 Borrow BorrowMut
    // 13.9 From Into
    use std::net::Ipv4Addr;
    fn ping<A>(address: A) -> std::io::Result<bool>
    where
        A: Into<Ipv4Addr>,
    {
        let ipv4_address = address.into();
        println!("Pinging {}...", ipv4_address);
        Ok(true)
    }

    ping(Ipv4Addr::new(127, 0, 0, 1)).unwrap();
    ping([8, 8, 8, 8]).unwrap();
    ping(0xd076eb94_u32).unwrap();

    let addr1 = Ipv4Addr::from([127, 0, 0, 1]);
    let addr2 = Ipv4Addr::from(0xd076eb94_u32);
    println!("addr1: {:?}", addr1);
    println!("addr2: {:?}", addr2);

    let text = "Beautiful Soup".to_string();
    let bytes: Vec<u8> = text.into();
    println!("bytes: {:?}", bytes);

    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;
    #[allow(dead_code)]
    fn parse_i32_bytes(b: &[u8]) -> GenericResult<i32> {
        Ok(std::str::from_utf8(b)?.parse::<i32>()?)
    }
    
    let huge = 2_000_000_000_000_i64;
    let smaller = huge as i32;
    println!("{} as i32: {}", huge, smaller);
}
