use rayon::prelude::*;
use std::sync::Arc;
use std::{io, thread};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("19.1 fork, join parallel");

    fn process_files(filenames: Vec<String>) -> io::Result<()> {
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

    println!("19.1.1 spawn & join");
    let handle = thread::spawn(|| {
        println!("Hello from the spawned thread!");
    });
    handle.join().unwrap();

    fn split_vec_into_chunks(vec: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
        vec.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
    }

    #[allow(dead_code)]
    fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
        const NTHREADS: usize = 8;
        let worklists = split_vec_into_chunks(filenames, NTHREADS);
        let mut thread_handles = Vec::new();
        for worklist in worklists {
            thread_handles.push(thread::spawn(move || process_files(worklist)))
        }

        for handle in thread_handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }

    println!("19.1.2 error handling between threads");

    println!("19.1.3 shared const between threads");
    type GigabyteMap = String;

    #[allow(unused_variables)]
    fn process_files_with_glossary(
        filenames: Vec<String>,
        glossary: &GigabyteMap,
    ) -> io::Result<()> {
        for document in filenames {
            let text = load(&document)?;
            let result = process(&text)?;
            save(&document, &result)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn process_files_in_parallel_with_glossary(
        filenames: Vec<String>,
        glossary: Arc<GigabyteMap>,
    ) -> io::Result<()> {
        const NTHREADS: usize = 8;
        let worklists = split_vec_into_chunks(filenames, NTHREADS);
        let mut thread_handles = Vec::new();
        for worklist in worklists {
            let glossary_for_child = glossary.clone();
            thread_handles.push(thread::spawn(move || {
                process_files_with_glossary(worklist, &glossary_for_child)
            }))
        }

        for handle in thread_handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }

    println!("19.1.4 Rayon");
    #[allow(unused_variables)]
    fn process_file(filenames: &String, glossary: &GigabyteMap) -> io::Result<()> {
        Ok(())
    }

    #[allow(dead_code)]
    fn process_files_in_rayon(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()> {
        filenames
            .par_iter()
            .map(|filename| process_file(filename, glossary))
            .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
            .unwrap_or(Ok(()))
    }

    println!("19.1.5 mandelbrot returns");

    Ok(())
}
