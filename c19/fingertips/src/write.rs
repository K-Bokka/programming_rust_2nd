use crate::index::InMemoryIndex;
use crate::tmp::TmpDir;
use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct IndexFileWriter {
    offset: u64,
    writer: io::BufWriter<File>,
    contents_buf: Vec<u8>,
}

impl IndexFileWriter {
    pub fn new(mut f: io::BufWriter<File>) -> io::Result<IndexFileWriter> {
        const HEADER_SIZE: u64 = 8;
        f.write_u64::<LittleEndian>(0)?;
        Ok(IndexFileWriter {
            offset: HEADER_SIZE,
            writer: f,
            contents_buf: vec![],
        })
    }

    pub fn write_main(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)?;
        self.offset += buf.len() as u64;
        Ok(())
    }

    pub fn write_contents_entry(&mut self, term: String, df: u32, offset: u64, nbytes: u64) {
        self.contents_buf.write_u64::<LittleEndian>(offset).unwrap();
        self.contents_buf.write_u64::<LittleEndian>(nbytes).unwrap();
        self.contents_buf.write_u32::<LittleEndian>(df).unwrap();
        let bytes = term.bytes();
        self.contents_buf
            .write_u32::<LittleEndian>(bytes.len() as u32)
            .unwrap();
        self.contents_buf.extend(bytes);
    }

    pub fn finish(mut self) -> io::Result<()> {
        let contents_start = self.offset;
        self.writer.write_all(&self.contents_buf)?;
        println!(
            "{} bytes main, {} bytes total",
            contents_start,
            contents_start + self.contents_buf.len() as u64
        );
        self.writer.seek(io::SeekFrom::Start(0))?;
        self.writer.write_u64::<LittleEndian>(contents_start)?;
        Ok(())
    }
}
pub fn write_index_to_tmp_file(index: InMemoryIndex, tmp_dir: &mut TmpDir) -> io::Result<PathBuf> {
    let (filename, f) = tmp_dir.create()?;
    let mut writer = IndexFileWriter::new(f)?;

    let mut index_as_vec: Vec<_> = index.map.into_iter().collect();
    index_as_vec.sort_by(|&(ref a, _), &(ref b, _)| a.cmp(b));

    for (term, hits) in index_as_vec {
        let df = hits.len() as u32;
        let start = writer.offset;
        for buffer in hits {
            writer.write_main(&buffer)?;
        }
        let stop = writer.offset;
        writer.write_contents_entry(term, df, start, stop - start);
    }

    writer.finish()?;
    println!("wrote file {:?}", filename);
    Ok(filename)
}
