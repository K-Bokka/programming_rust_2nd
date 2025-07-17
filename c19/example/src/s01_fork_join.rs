use std::io;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("19.1 fork, join parallel");

    #[allow(dead_code)]
    fn process_file(filenames: Vec<String>) -> io::Result<()> {
        for document in filenames {
            let text = load(&document)?;
            let result = process(&text)?;
            save(&document, &result)?;
        }
        Ok(())
    }

    fn load(doc: &str) -> io::Result<String> {
        // load document
        Ok(doc.to_string())
    }
    fn process(text: &str) -> io::Result<String> {
        // process text
        Ok(text.to_string())
    }
    fn save(_doc: &str, _text: &str) -> io::Result<()> {
        // save text to doc
        Ok(())
    }

    Ok(())
}
