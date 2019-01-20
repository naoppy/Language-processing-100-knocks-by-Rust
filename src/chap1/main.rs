extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    println!("{}", reverse("あいうえお"));
    println!("{}", odd_chars("パタトクカシー"));
    println!("{}", concat_alternately("パトカー", "タクシー"));
    pi_from_text().iter().for_each(|u| print!("{}", u));
    println!("");
    for (element, len) in &element_symbols() {
        println!("{} : {}", element, len);
    }
    for s in &character_n_gram("I am an NLPer", 2) {
        println!("{}", s);
    }
    print_set();
    println!("{}", generate_template_text(12, "気温", 22.4));
    println!("{}", encrypt("Hello, World"));
    println!("{}", typoglycemia("I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind ."));
}

fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

fn odd_chars(text: &str) -> String {
    text.chars().step_by(2).collect()
}

fn concat_alternately(left: &str, right: &str) -> String {
    let mut result = String::new();
    left.chars().zip(right.chars()).for_each(|(l, r)| {
        result.push(l);
        result.push(r);
    });
    result
}

fn pi_from_text() -> Vec<usize> {
    let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    text.replace(",", "")
    .replace(".", "")
    .split_whitespace()
    .map(|s| s.len())
    .collect()
}

fn element_symbols() -> HashMap<String, usize> {
    let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let mut result = HashMap::new();
    text.replace(".", "")
    .split_whitespace()
    .enumerate()
    .for_each(|(i, s)| {
        let symbol = match i {
            0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 =>
            s.get(0..1).unwrap(),
            _ => s.get(0..2).unwrap()
        };
        result.insert(symbol.to_string(), i+1);
    });
    result
}

fn character_n_gram(text: &str, n: usize) -> Vec<String> {
    let max = text.len() - n + 1;
    let mut result = Vec::new();
    for i in 0..max {
        result.push(text.get(i..i+n).unwrap().to_string())
    }
    result
}

fn print_set() {
    let paraparaparadise = character_n_gram("paraparaparadise", 2)
    .into_iter().collect::<HashSet<String>>();
    let paragraph = character_n_gram("paragraph", 2)
    .into_iter().collect::<HashSet<String>>();
    println!("和集合={:?}, 積集合={:?}, 差集合={:?}, find 'se'={:?}",
         &paraparaparadise | &paragraph,
         &paraparaparadise & &paragraph,
         &paraparaparadise - &paragraph,
         (&paraparaparadise | &paragraph).contains("se")
    );
}

fn generate_template_text<X: Display, Y: Display, Z: Display>(x: X, y: Y, z: Z) -> String {
    format!("{}時の{}は{}", x, y, z)
}

fn encrypt(text: &str) -> String {
    text.chars().map(|c| if c.is_ascii_lowercase() {
        (219 - c as u8) as char
    } else { c })
    .collect()
}

fn typoglycemia(text: &str) -> String {
    text.split_whitespace().map(|s| {
        if s.len() < 5 {
            s.to_string()
        } else {
            let (head, remaining) = s.split_at(1);
            let (body, tail) = remaining.split_at(remaining.len() - 1);
            let mut body: Vec<_> = body.chars().collect();
            body.shuffle(&mut thread_rng());
            head.to_string() + &body.into_iter().collect::<String>() + tail
        }
    }).fold(String::new(), |result, s| if result == "" { s } else { result + " " + &s})
}