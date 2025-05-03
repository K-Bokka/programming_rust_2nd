use std::fmt::Debug;
use std::ops::Sub;
use std::ops::{IndexMut, Mul};

fn main() {
    println!("Chapter 12");

    // 12.
    #[derive(Debug, Copy, Clone, Eq)]
    struct Complex<T> {
        re: T,
        im: T,
    }
    let c = Complex { re: 1, im: 2 };
    println!("{:?}", c);

    // 12.1
    use std::ops::Add;
    assert_eq!(4.125f32.add(5.25), 9.375);
    assert_eq!(10.add(10), 10 + 10);

    impl<L, R> Add<Complex<R>> for Complex<L>
    where
        L: Add<R>,
    {
        type Output = Complex<L::Output>;
        fn add(self, rhs: Complex<R>) -> Self::Output {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }

    let d = Complex { re: 3, im: 4 };
    println!("{:?}", c + d);

    // 12.1.1
    use std::ops::Neg;
    impl<T> Neg for Complex<T>
    where
        T: Neg<Output = T>,
    {
        type Output = Complex<T>;
        fn neg(self) -> Self::Output {
            Complex {
                re: -self.re,
                im: -self.im,
            }
        }
    }
    println!("{:?}", -c);

    // 12.1.2
    // 12.1.3
    use std::ops::AddAssign;

    impl<T> AddAssign for Complex<T>
    where
        T: AddAssign<T>,
    {
        fn add_assign(&mut self, rhs: Self) {
            self.re += rhs.re;
            self.im += rhs.im;
        }
    }
    let mut e = Complex { re: 1, im: 2 };
    e += c;
    println!("{:?}", e);

    // 12.2
    let x = 1;
    let y = 1;

    assert_eq!(x == y, x.eq(&y));
    assert_eq!(x != y, x.ne(&y));

    impl<T: PartialEq> PartialEq for Complex<T> {
        fn eq(&self, other: &Self) -> bool {
            self.re == other.re && self.im == other.im
        }
    }
    assert_ne!(c, d);

    impl<T> Mul for Complex<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
    {
        type Output = Complex<T>;
        fn mul(self, rhs: Self) -> Self::Output {
            Complex {
                re: (self.re * rhs.re) - (self.im * rhs.im),
                im: (self.re * rhs.im) + (self.im * rhs.re),
            }
        }
    }

    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_eq!(x * y, Complex { re: 0, im: 29 });

    assert_ne!("ungula", "ungulate");
    assert!("ungula".ne("ungulate"));

    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
    assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);

    // 12.3
    #[derive(Debug, PartialEq)]
    struct Interval<T> {
        lower: T, // Inclusive
        upper: T, // Exclusive
    }

    use std::cmp::Ordering;
    use std::cmp::PartialOrd;
    impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
        fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
            if self == other {
                Some(Ordering::Equal)
            } else if self.lower >= other.upper {
                Some(Ordering::Greater)
            } else if self.upper <= other.lower {
                Some(Ordering::Less)
            } else {
                None
            }
        }
    }

    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });

    let left = Interval {
        lower: 10,
        upper: 30,
    };
    let right = Interval {
        lower: 20,
        upper: 40,
    };
    assert!(!(left < right));
    assert!(!(left > right));

    let mut intervals = vec![
        Interval {
            lower: 10,
            upper: 30,
        },
        Interval {
            lower: 5,
            upper: 10,
        },
        Interval {
            lower: 20,
            upper: 25,
        },
    ];
    intervals.sort_by_key(|i| i.lower);
    println!("{:?}", intervals);

    use std::cmp::Reverse;
    intervals.sort_by_key(|i| Reverse(i.upper));
    println!("{:?}", intervals);

    // 12.4
    use std::collections::HashMap;
    let mut m = HashMap::new();
    m.insert("十", 10);
    m.insert("百", 100);
    m.insert("千", 1000);
    m.insert("万", 1_0000);
    m.insert("億", 1_0000_0000);

    assert_eq!(m["十"], 10);
    assert_eq!(m["千"], 1000);

    use std::ops::Index;
    assert_eq!(*m.index("十"), 10);
    assert_eq!(*m.index("千"), 1000);

    let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
    desserts[0].push_str(" (fictional)");
    desserts[1].push_str(" (real)");
    println!("{:?}", desserts);

    #[derive(Debug)]
    struct Image<P> {
        width: usize,
        pixels: Vec<P>,
    }

    impl<P: Default + Copy> Image<P> {
        fn new(width: usize, height: usize) -> Self {
            Image {
                width,
                pixels: vec![P::default(); width * height],
            }
        }
    }

    impl<P> Index<usize> for Image<P> {
        type Output = [P];
        fn index(&self, row: usize) -> &[P] {
            let start = row * self.width;
            &self.pixels[start..start + self.width]
        }
    }

    impl<P> IndexMut<usize> for Image<P> {
        fn index_mut(&mut self, row: usize) -> &mut [P] {
            let start = row * self.width;
            &mut self.pixels[start..start + self.width]
        }
    }

    let image: Image<usize> = Image::new(10, 10);
    println!("{:?}", &image[5]);
}
