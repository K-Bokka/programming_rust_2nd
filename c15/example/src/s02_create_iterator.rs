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
}
