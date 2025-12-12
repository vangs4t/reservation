use std::{cmp::Ordering, io};

use random_number::rand;

mod todolist;
mod product_type;

fn main() {
    guest_number();
}

struct Todolist;

impl Todolist {
    fn tambah(todos: String) -> Vec<String>{
        let mut hasil = vec![];
        hasil.push(todos);
        hasil
    }
}

fn guest_number(){
    let num = random_number::random!(..=10);
    let mut input = String::new();

    println!("Guest the number! 1 - 10");

    let _ = io::stdin().read_line(&mut input);

    let gues: u32 = input.trim().parse().expect("msg");
    println!("{num}");

    match gues.cmp(&num) {
        Ordering::Less => println!("Kurang"),
        Ordering::Greater => println!("Lebihh!"),
        Ordering::Equal => println!("Kamu menang")
    }

}