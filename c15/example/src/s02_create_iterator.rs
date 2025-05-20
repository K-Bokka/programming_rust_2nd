pub fn run() {
    println!("15.2 Create Iterator");

    // 15.2.1 iter, iter_mut method
    let v = vec![4, 20, 12, 8, 6];
    let mut iter = v.iter();
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&12));
    assert_eq!(iter.next(), Some(&8));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);

    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("/Users/ak-yama/git/k-bokka/programing_rust_2nd/README.md");
    let mut path_iter = path.iter();
    assert_eq!(path_iter.next(), Some(OsStr::new("/")));
    assert_eq!(path_iter.next(), Some(OsStr::new("Users")));
    assert_eq!(path_iter.next(), Some(OsStr::new("ak-yama")));

    // 15.2.2 IntoIterator
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky with Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky with Diamonds".to_string()));
    assert_eq!(it.next(), None);

    // 消費されているので空っぽになっている
    // assert!(favorites.is_empty()); // error[E0382]: borrow of moved value: `favorites`

    use std::fmt::Debug;

    fn dump<T, U>(t: T)
    where
        T: IntoIterator<Item = U>,
        U: Debug,
    {
        for u in t {
            println!("{:?}", u);
        }
    }

    let v = vec![1, 2, 3];
    dump(v);

    // 15.2.3 from_fn, successors
    use rand::random;
    use std::iter::from_fn;
    let lengths: Vec<f64> = from_fn(|| Some(random::<f64>() - random::<f64>().abs()))
        .take(10)
        .collect();
    println!("{:?}", lengths);

    use num::Complex;
    use std::iter::successors;

    #[allow(dead_code)]
    fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
        let zero = Complex { re: 0.0, im: 0.0 };
        successors(Some(zero), |&z| Some(z * z + c))
            .take(limit)
            .enumerate()
            .find(|&(_i, z)| z.norm() > 4.0)
            .map(|(i, _z)| i)
    }

    fn fibonacci() -> impl Iterator<Item = usize> {
        let mut state = (0, 1);
        from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }
    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );

    // 15.2.4 drain
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(inner, "art");
    assert_eq!(outer, "Eh");
}
