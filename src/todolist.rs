use core::num;
use std::{collections::HashMap, vec};

use regex::Regex;

struct Todolist{
    todo: Vec<String>
}


impl Todolist{

fn add_todo(&mut self, input: String){

    // disini saya menggunakan perintah regex (r"\w+") untuk menemukan satu atau karakter lebih kata
    let mut hasil: Vec<String> = Regex::new(r"\w+")
    .unwrap() 
    .find_iter(&input)
    .map(|x| x.as_str().to_string())
    .collect();

    self.todo.append(&mut hasil);
}

fn remove_todo(&mut self, number: String){
    // memparsing input string number dan mengabaikan error
    let mut temp: Vec<usize> = number
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    // menghapus dari index yang terbesar ke terkecil sekaligus mengcompare
    temp.sort_unstable_by(|a, b| b.cmp(a));

    for mut idx in temp {
        if idx <= self.todo.len() {
            idx -=1;
            self.todo.remove(idx);
        } else {
            println!("Penghapusan gagal")
        }
    }
}

fn done(&self){

}

fn list(&self){
        let mut num = 1;
    
        for value in &self.todo {
            println!("{}. {}", num, value);
            num+=1;
        }
}

fn display(&mut self, input: String){
    if input.contains("todo"){
        let replacement = input.replace("todo", " ");
        if replacement.contains("add"){
            let adds = replacement.replace("add", " ");
            self.add_todo(adds);

            println!("Add Succes");
        } else if replacement.contains("done") {
            let done = replacement.replace("done", " ");
            self.remove_todo(done);

            println!("Remove Succes");
        } else {
            self.list();
        }
    } else {
        println!("Input tidak valid");
    }
}





}

#[test]
fn dones() {
    let mut hasil = Todolist{
        todo : vec!["abjad".to_string(), "Kanjut".to_string(), "alhasil".to_string(),"vupang".to_string()]
    };

    hasil.display("todo add halo sayadisini".to_string());
    hasil.display("todo done 4".to_string());
    hasil.display("todo".to_string());
}

