use core::num;
use std::{collections::HashMap, vec};

use regex::Regex;

struct Todolist{
    todo: Vec<String>
}


impl Todolist{

fn add_todo(&mut self, input: String){

    // disini saya menggunakan perintah regex (r"\w+") untuk menemukan satu atau karakter lebih kata
    self.todo = Regex::new(r"\w+")
    .unwrap() 
    .find_iter(&input)
    .map(|x| x.as_str().to_string())
    .collect();
}

fn remove_todo(&mut self, number: String){
    // Parse indices from input string, ignore parse errors
    let mut indices: Vec<usize> = number
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    // Remove from highest index to lowest so earlier removals don't shift later indices
    indices.sort_unstable_by(|a, b| b.cmp(a));

    for idx in indices {
        if idx < self.todo.len() {
            self.todo.remove(idx);
        }
    }
}

fn displays(&self){
        let mut num = 1;
    
        for value in &self.todo {
            println!("{}. {}", num, value);
            num+=1;
        }
}





}

#[test]
fn dones() {
    let mut hasil = Todolist{
        todo : vec!["abjad".to_string(), "Kanjut".to_string(), "alhasil".to_string()]
    };

    hasil.remove_todo("1 ".to_string());
    hasil.displays();
}

