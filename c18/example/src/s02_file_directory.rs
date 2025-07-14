use std::ffi::OsStr;
use std::path::Path;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("18.2 File and Directory");

    println!("\n18.2.1 OsStr and Path");
    println!("\n18.2.2 Path & PathBuf method");

    let home_dir = Path::new("/home/rust");
    assert_eq!(Path::new("/home/rust/main.rs").parent(), Some(home_dir));

    assert_eq!(
        Path::new("/home/rust/main.rs").file_name(),
        Some(OsStr::new("main.rs"))
    );
    let path1 = Path::new("/home/rust/");
    assert_eq!(path1.join("words"), Path::new("/home/rust/words"));

    let any_path = Path::new("rust/words");
    let abs_path = std::env::current_dir()?.join(any_path);
    println!("abs_path: {:?}", abs_path);
    println!("{:?}", abs_path.components());

    let file = Path::new("/home/rust/calendars/calendar-18x18.pdf");
    assert_eq!(
        file.ancestors().collect::<Vec<_>>(),
        vec![
            Path::new("/home/rust/calendars/calendar-18x18.pdf"),
            Path::new("/home/rust/calendars"),
            Path::new("/home/rust"),
            Path::new("/home"),
            Path::new("/"),
        ]
    );

    if let Some(file_str) = file.to_str() {
        println!("file_str: {}", file_str);
    }

    Ok(())
}
