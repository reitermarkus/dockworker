use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::iter::{self};

use rand::Rng;
use rand;

pub fn equal(patha: &Path, pathb: &Path) -> bool {
  let filea = File::open(patha).unwrap();
  let fileb = File::open(pathb).unwrap();
  filea.bytes()
       .map(|e| e.ok())
       .eq(fileb.bytes().map(|e| e.ok()))
}

// generate a file on path which is constructed from size chars alphanum seq
pub fn generate(path: &Path, size: usize) -> io::Result<()> {
  let mut rng = rand::thread_rng();
  let mut file = File::create(path)?;
  let vec: String = iter::repeat(())
    .map(|_| rng.sample(rand::distributions::Alphanumeric))
    .take(size)
    .collect();
  file.write_all(vec.as_bytes())
}
