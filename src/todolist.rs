
use std::io::{self, stdin};

use regex::Regex;

struct Todolist{
    todo: Vec<String>
}


impl Todolist{

fn new() -> Todolist{
    let results = Todolist{
        todo : vec![]
    };

    results
}

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

fn display(&mut self, input: String) -> bool{
    // Proses satu perintah saja per pemanggilan (hindari loop tak berujung)
    if input.contains("todo") {
        let replacement = input.replace("todo", " ");
        let normalized = replacement.to_lowercase();

        if normalized.contains("add") {
            if let Some(idx) = normalized.find("add") {
                // ambil isi setelah kata 'add'
                let adds = replacement[idx + 3..].trim().to_string();
                self.add_todo(adds);
                println!("Add Succes");
                true
            } else {
                false
            }
        } else if normalized.contains("done") {
            if let Some(idx) = normalized.find("done") {
                // ambil nomor setelah kata 'done'
                let done = replacement[idx + 4..].trim().to_string();
                self.remove_todo(done);
                println!("Remove Succes");
            }
            true
        } else if replacement.trim().eq_ignore_ascii_case("exit") {
            println!("Semoga tugasnya cepat selesai!");
            false
        } else if replacement.trim().is_empty() {
            // hanya `todo` -> tampilkan daftar
            self.list();
            true
        } else {
            println!("Input tidak valid"); 
            true
        }
    } else {
        println!("Input tidak valid");
        true
    }
}


}

pub fn todolist_app(){
    let mut input = String::new();
    let mut todolist = Todolist::new();
    let mut kondisi = true;

    while kondisi {
        if !input.eq_ignore_ascii_case("exit".trim()) {
            stdin().read_line(&mut input).unwrap();
            kondisi = todolist.display(input.clone());
            input.clear();

        } else {
            kondisi = false;
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

