use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, ErrorKind, Error, Write};
use std::io;

fn main() {
    println!("{} lines", count_lines(Path::new("hightemp.txt")).unwrap());
}

// 10
fn count_lines(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut counter = 0;
    br.lines().for_each(|_| counter += 1);
    Ok(counter)
}

// 11
fn tab_to_space(path: &Path, tab_width: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let spaces: String = " ".repeat(tab_width);
    Ok(br.lines().map(|s| match s {
        Ok(s) => s.replace("\t", &spaces) + "\n",
        Err(_) => "\0".to_string()
    }).collect())
}

//12
fn get_col(source: &Path, out: &Path, column_number: usize) -> io::Result<()> {
    let source = File::open(source)?;
    let out = OpenOptions::new().write(true).create(true).truncate(true).open(out)?;
    let br = BufReader::new(source);
    let mut bw = BufWriter::new(out);
    br.lines().map(|line| {
        if let Ok(line) = line {
            match line.split_whitespace().nth(column_number) {
                Some(word) => Ok(word.to_string() + "\n"),
                None => Err(Error::new(ErrorKind::NotFound, format!("the column {} is not found.", column_number)))
            }
        } else {
            line
        }
    }).for_each(|col| {
        let _ = col.and_then(|col| bw.write(col.as_bytes()));
    });
    Ok(())
}