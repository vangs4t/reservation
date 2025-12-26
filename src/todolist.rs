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

fn tambah(&mut self, input: String) -> Vec<String>{

    // disini saya menggunakan perintah regex (r"\w+") untuk menemukan satu atau karakter lebih kata
    self.todo = Regex::new(r"\w+")
    .unwrap() 
    .find_iter(&input)
    .map(|x| x.as_str().to_string())
    .collect();

    self.todo.clone()
}

fn done(&self,number: String) -> Vec<u32>{

    if number.contains("done") {

        let hapus = number[4..].to_string();

        let hasil: Vec<u32> = hapus.split_ascii_whitespace()
        .map(|x|x.parse().unwrap())
        .collect();

        hasil

    } else {

        vec![]

    }

}

fn displays(&self){

    if self.todo.contains("todo") {

        let hapus = self.todo[4..].to_string();
        let hasil = self.tambah(hapus);
        let mut num = 1;
    
        for value in hasil {
            println!("{}. {}", num, value);
            num+=1;
        }

    } else {

        println!("invalid Command");
    
    }
    }
}
