use std::{clone, collections::HashMap};

#[derive(Debug, Clone)]
struct Todoitem{
    id: u32,
    deskripsi_tugas: String,
    status: String
}

#[derive(Debug, Clone)]
struct Todolist {
    item: HashMap<u32, Todoitem>,
    next: u32
}

impl Todolist {
    fn new(){
        let mut hasil = Todolist{
            item: HashMap::new(),
            next: 1
        };
    }
    fn add_item(input: String) -> HashMap<u32, Todoitem>{
        let item = Todoitem{
            id: 233,
            deskripsi_tugas: input,
            status : "Pending".to_string()
        };
        let mut baru = HashMap::new();
        baru.insert(item.id, item);
        baru
    }
}

#[derive(Debug, Clone)]
enum status{
    Pending,
    Done
}


