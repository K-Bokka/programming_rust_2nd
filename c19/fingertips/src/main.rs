mod index;

use index::InMemoryIndex;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::{fs, thread};

fn main() {
    println!("https://github.com/ProgrammingRust/fingertips");
}

#[allow(dead_code)]
fn start_file_reader_thread(
    documents: Vec<PathBuf>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename)?;
            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });
    (receiver, handle)
}

#[allow(dead_code)]
fn start_file_indexing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::from_single_document(doc_id, text);
            if sender.send(index).is_err() {
                break;
            }
        }
    });
    (receiver, handle)
}
