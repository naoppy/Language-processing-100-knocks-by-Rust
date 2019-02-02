use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, ErrorKind, Error, Write};
use std::io;
use std::collections::{HashSet, HashMap};

fn main() {
    println!("{} lines", count_lines(Path::new("hightemp.txt")).unwrap());
}

// 10
/// `path`で指定されたファイルの行数を返す
fn count_lines(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut counter = 0;
    br.lines().for_each(|_| counter += 1);
    Ok(counter)
}

// 11
/// `path`で指定されたファイルのタブを`tab_width`の数のスペースに置換する
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
/// `source`の各行について、空白で分けられた`column_number`目の列を`out`に書き出す
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

//13
/// `source1`と`source2`を行ごとにタブをデミリタとして連結する
fn merge_columns(source1: &Path, source2: &Path) -> io::Result<String> {
    let source1 = File::open(source1)?;
    let source2 = File::open(source2)?;
    let br1 = BufReader::new(source1);
    let br2 = BufReader::new(source2);
    Ok(br1.lines().zip(br2.lines()).map(|(col1, col2)| {
        col1.unwrap() + "\t" + &col2.unwrap() + "\n"
    }).collect())
}

//14
/// `path`で指定されたファイルの先頭から`n`行を返す
fn heads(path: &Path, n: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    br.lines().take(n).map(|line| line.and_then(|line| Ok(line + "\n"))).collect()
}

//15
/// `path`で指定されたファイルの末尾から`n`行を返す
fn tails(path: &Path, n: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines = br.lines().collect::<Vec<_>>();
    let n_lines = lines.into_iter().rev().take(n).collect::<Vec<_>>();
    n_lines.into_iter().rev().map(|line| line.and_then(|line| Ok(line + "\n"))).collect()
}

//16
/// `path`で指定されたファイルを行単位で`n`分割したベクタを返します
fn split_file(path: &Path, n: usize) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines = br.lines().collect::<io::Result<Vec<_>>>();
    lines.and_then(|lines| Ok(lines.chunks(n).map(|chunk| chunk.join("\n")).collect()))
}

//17
/// 
fn get_column_differences(path: &Path, n: usize) -> io::Result<HashSet<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut result = HashSet::new();
    br.lines().map(|line|
        line.and_then(|line| {
            line.split_whitespace().map(|word| word.to_string()).nth(n)
            .ok_or(Error::new(ErrorKind::NotFound, format!("the column {} is not found.", n)))
        })
    ).for_each(|line| match line {
        Ok(line) => { result.insert(line.to_string()); },
        Err(e) => eprintln!("{}", e)
    });
    Ok(result)
}

//18
/// 
fn sort_by_column(path: &Path, n: usize) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines = br.lines().collect::<io::Result<Vec<_>>>();
    lines.and_then(|mut lines| {
        lines.sort_by(|a, b| a.split_whitespace().nth(n).cmp(&b.split_whitespace().nth(n)));
        Ok(lines)
    })
}

//19
/// 
fn sort_by_frequency(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines = br.lines().collect::<io::Result<Vec<_>>>();
    let mut counter: HashMap<String, usize> = HashMap::new();
    lines.and_then(|lines|
        Ok(lines.into_iter().for_each(|line| {
            let key = line.split_whitespace().next().unwrap_or("");
            if counter.contains_key(key) {
                *counter.get_mut(key).unwrap() += 1
            } else {
                counter.insert(key.to_string(), 1);
            }
        }))
    ).and_then(|_| {
        let mut tmp = counter.into_iter().collect::<Vec<_>>();
        tmp.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(tmp.into_iter().map(|(k, _)| k).collect())
    })
}