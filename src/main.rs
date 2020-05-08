use rand::seq::IteratorRandom;
use std::io::{Result, Seek, SeekFrom, BufReader, BufRead};
use std::fs::{File, metadata, read_dir};
use rand::Rng;

fn get_random_fortune_file() -> String {
    let path = "/Users/rohit/play/rust-apps/fortune/src/datfiles";
    let entries = read_dir(path).unwrap()
        .map(|result| result.map(|e| e.path()))
        .collect::<Result<Vec<_>>>().unwrap();
    let mut rng = rand::thread_rng();
    entries.iter().choose(&mut rng).unwrap().to_str().unwrap().to_string()
}

fn get_fortune() -> String {
    let file_path = get_random_fortune_file();
    let mut file = BufReader::new(File::open(&file_path).unwrap());
    let metadata = metadata(&file_path).unwrap();

    let mut rng = rand::thread_rng();
    let mut random_byte = rng.gen_range(0, metadata.len());
    // seek to an offset
    file.seek(SeekFrom::Start(random_byte)).unwrap();

    // find the first % by reading from offset
    let mut buf = vec![];
    match file.read_until(b'%', &mut buf) {
        Ok(0) => {
            random_byte = 0;
            0
        },
        Ok(size) => {
            random_byte += size as u64 + 1;
            size
        },
        Err(_) => {
            random_byte = 0;
            0
        }
    };

    // update the seek position to start read from this location
    file.seek(SeekFrom::Start(random_byte)).unwrap();

    let mut buf = vec![];
    file.read_until(b'%', &mut buf).unwrap();
    buf.pop();
    buf.pop();

    return String::from_utf8(buf).unwrap();
}

fn main() -> Result<()> {
    println!("{}", get_fortune());
    Ok(())
}

#[test]
fn get_fortune_test() {
    assert!(!get_fortune().is_empty());
}
