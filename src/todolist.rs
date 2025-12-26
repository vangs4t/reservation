use core::num;
use std::vec;

use regex::Regex;

struct Todolist{
    todo: Vec<String>
}


impl Todolist{

fn new(&self) -> Todolist{
    Todolist { todo: vec![] }
}

fn tambah(&mut self, input: String){

    // disini saya menggunakan perintah regex (r"\w+") untuk menemukan satu atau karakter lebih kata
    self.todo = Regex::new(r"\w+")
    .unwrap() 
    .find_iter(&input)
    .map(|x| x.as_str().to_string())
    .collect();
}

fn get_number(&self,number: String) -> Vec<u32>{
        let hasil: Vec<u32> = number.split_ascii_whitespace()
        .map(|x|x.parse().unwrap())
        .collect();

        hasil
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
        todo : vec!["abjad".to_string(), "Kanjut".to_string()]
    };

    println!("{:?}", hasil.get_number("0 1".to_string()));
    hasil.displays();
}

