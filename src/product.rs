use std::collections::HashMap;

#[derive(Debug)]
struct Product{
    id_produk: u32,
    nama_produk: String,
    stok: u32,
    harga_produk: f64,
}

impl Product {
    fn add_product(&self, nama: String,stoks: u32, harga: f64) -> HashMap<u32,Product>{
        let mut result = HashMap::new();
        result.insert(self.id_produk, Product { 
            id_produk: self.id_produk, 
            nama_produk: nama, 
            stok: stoks,
            harga_produk: harga });
        result
    }
}

