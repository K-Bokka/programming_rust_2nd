mod index;
mod merge;
mod read;
mod tmp;
mod write;

use crate::merge::FileMerge;
use crate::tmp::TmpDir;
use crate::write::write_index_to_tmp_file;
use index::InMemoryIndex;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::channel;
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

#[allow(dead_code)]
fn start_in_memory_merge_thread(
    file_indices: mpsc::Receiver<InMemoryIndex>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = channel();

    let handle = thread::spawn(move || {
        let mut accumulated_index = InMemoryIndex::new();
        for fi in file_indices {
            accumulated_index.merge(fi);
            if accumulated_index.is_large() {
                if sender.send(accumulated_index).is_err() {
                    return;
                }
                accumulated_index = InMemoryIndex::new();
            }
        }
        if !accumulated_index.is_empty() {
            let _ = sender.send(accumulated_index);
        }
    });

    (receiver, handle)
}

#[allow(dead_code)]
fn start_index_writer_thread(
    big_indexes: mpsc::Receiver<InMemoryIndex>,
    output_dir: &Path,
) -> (mpsc::Receiver<PathBuf>, thread::JoinHandle<io::Result<()>>) {
    let (sender, receiver) = channel();

    let mut tmp_dir = TmpDir::new(output_dir);
    let handle = thread::spawn(move || {
        for index in big_indexes {
            let file = write_index_to_tmp_file(index, &mut tmp_dir)?;
            if sender.send(file).is_err() {
                break;
            }
        }
        Ok(())
    });

    (receiver, handle)
}

#[allow(dead_code)]
fn merge_index_files(files: mpsc::Receiver<PathBuf>, output_dir: &Path) -> io::Result<()> {
    let mut merge = FileMerge::new(output_dir);
    for file in files {
        merge.add_file(file)?;
    }
    merge.finish()
}
