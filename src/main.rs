use std::{cmp::Ordering, collections::{HashMap, HashSet, btree_map::Values}, io};

use random_number::rand;
use colored::Colorize;

mod todolist;
mod product_type;

fn main() {
    let number = 12345;
    // Menggunakan .strikethrough() pada string yang diformat
    println!("Nomor yang dicoret: {}", number.to_string().strikethrough());

    // Contoh lain dengan warna dan coretan
    let old_price = "99.99".strikethrough().blue();
    let new_price = "49.99".green().bold();
    println!("Harga: {} {}", old_price, new_price);
}

struct Todolist{
    description: String
}

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

    let gues: u32 = input.trim().parse().unwrap();
    println!("{num}");

    match gues.cmp(&num) {
        Ordering::Less => println!("Kurang"),
        Ordering::Greater => println!("Lebihh!"),
        Ordering::Equal => println!("Kamu menang")
    }

}
struct Army{
    unit: Vec<i32>,
    power: Vec<i32>
}
fn good_vs_evil(good: &str, evil: &str) -> String {

  let kebaikan = Army{
    unit: good.split_ascii_whitespace().filter_map(|x|x.parse::<i32>().ok()).collect(),
    power: vec![1,2,3,3,4,10],
  };

  let mut results_good: i32 = 0;

  for i in 0..=5{
      results_good += kebaikan.power[i]* kebaikan.unit[i];
  }

  let kejelekan = Army{
    unit: evil.split_ascii_whitespace().filter_map(|x|x.parse::<i32>().ok()).collect(),
    power: vec![1,2,2,2,3,5,10],
  };
  
  let mut result_bad: i32 = 0;

  for i in 0..=6{
      result_bad += kejelekan.unit[i] * kejelekan.power[i];
  }

  if results_good > result_bad {
      "Battle Result: Good triumphs over Evil".to_string()
  } else if results_good < result_bad {
      "Battle Result: Evil eradicates all trace of Good".to_string()
  } else {
    "Battle Result: No victor on this battle field".to_string()
  }
}

fn do_test(good: &str, evil: &str, expected: &str) {
        let actual = good_vs_evil(good, evil);
        assert_eq!(
            actual, expected,
            "\n  Good: \"{good}\n  Evil: \"{evil}\"\nYour answer (left) is not the expected answer (right).",
        );
    }

    #[test]
    fn test_good_wins() {
        do_test(
            "1 0 1 0 0 0",
            "1 0 0 0 0 0 0",
            "Battle Result: Good triumphs over Evil",
        );
        do_test(
            "0 0 0 0 0 10",
            "0 0 0 0 0 0 0",
            "Battle Result: Good triumphs over Evil",
        );
    }

    #[test]
    fn test_evil_wins() {
        do_test(
            "1 0 0 0 0 0",
            "0 0 0 0 1 0 0",
            "Battle Result: Evil eradicates all trace of Good",
        );
        do_test(
            "0 0 0 0 0 0",
            "0 0 0 0 0 0 10",
            "Battle Result: Evil eradicates all trace of Good",
        );
    }

    #[test]
    fn test_tie() {
        do_test(
            "1 0 0 0 0 0",
            "1 0 0 0 0 0 0",
            "Battle Result: No victor on this battle field",
        );
        do_test(
            "0 0 0 0 0 10",
            "0 0 0 0 0 0 10",
            "Battle Result: No victor on this battle field",
        );
    }
    #[test]
    fn edge_case() {
        do_test(
            "0 0 0 0 0 0",
            "0 0 0 0 0 0 0",
            "Battle Result: No victor on this battle field",
        );
    }