use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct TmpDir {
    dir: PathBuf,
    n: usize,
}

impl TmpDir {
    pub fn new<P: AsRef<Path>>(dir: P) -> TmpDir {
        TmpDir {
            dir: dir.as_ref().to_owned(),
            n: 0,
        }
    }

    pub fn create(&mut self) -> io::Result<(PathBuf, io::BufWriter<fs::File>)> {
        let mut r#try = 1;
        loop {
            let filename = self
                .dir
                .join(PathBuf::from(format!("tmp{:08x}.dat", self.n)));
            self.n += 1;
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&filename)
            {
                Ok(f) => return Ok((filename, io::BufWriter::new(f))),
                Err(exc) => {
                    if r#try < 999 && exc.kind() == io::ErrorKind::AlreadyExists {
                        // keep going
                    } else {
                        return Err(exc);
                    }
                }
            }
            r#try += 1;
        }
    }
}
