mod todolist;
mod product_type;

fn main() {
    println!("{:?}",Todolist::tambah("Kanjut tebal".to_string()));
}

struct Todolist;

impl Todolist {
    fn tambah(todos: String) -> Vec<String>{
        let mut hasil = vec![];
        hasil.push(todos);
        hasil
    }
}