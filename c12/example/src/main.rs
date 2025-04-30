fn main() {
    println!("Chapter 12");

    // 12.
    #[derive(Debug, Copy, Clone)]
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
}
