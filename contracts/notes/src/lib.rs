#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan informasi produk
#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub stock: u32,
}

// Storage key untuk data shop
const SHOP_DATA: Symbol = symbol_short!("SHOP_DATA");

#[contract]
pub struct ShopContract;

#[contractimpl]
impl ShopContract {
    // 1. READ: Fungsi untuk mendapatkan list semua produk
    pub fn get_products(env: Env) -> Vec<Product> {
        // Ambil data produk dari storage, jika kosong kembalikan Vec baru
        return env.storage().instance().get(&SHOP_DATA).unwrap_or(Vec::new(&env));
    }

    // 2. CREATE: Fungsi untuk menambah produk baru ke toko
    pub fn add_product(env: Env, name: String, price: u64, stock: u32) -> String {
        // 1. Ambil data produk yang sudah ada dari storage
        let mut products: Vec<Product> = env.storage().instance().get(&SHOP_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object produk baru
        let product = Product {
            id: env.prng().gen::<u64>(), // Generate ID secara acak
            name: name,
            price: price,
            stock: stock,
        };
        
        // 3. Tambahkan produk baru ke list
        products.push_back(product);
        
        // 4. Simpan kembali list produk ke storage
        env.storage().instance().set(&SHOP_DATA, &products);
        
        return String::from_str(&env, "Produk berhasil ditambahkan");
    }

    // 3. UPDATE: Fungsi untuk membeli produk (mengurangi stok)
    pub fn buy_product(env: Env, id: u64, quantity: u32) -> String {
        // 1. Ambil data produk dari storage
        let mut products: Vec<Product> = env.storage().instance().get(&SHOP_DATA).unwrap_or(Vec::new(&env));

        // 2. Cari produk berdasarkan id menggunakan perulangan
        for i in 0..products.len() {
            let mut product = products.get(i).unwrap();
            
            if product.id == id {
                // 3. Cek apakah stok mencukupi untuk dibeli
                if product.stock >= quantity {
                    product.stock -= quantity; // Kurangi stok
                    products.set(i, product);  // Update produk dengan stok terbaru di dalam Vec
                    
                    // 4. Simpan kembali ke storage
                    env.storage().instance().set(&SHOP_DATA, &products);
                    return String::from_str(&env, "Berhasil membeli produk");
                } else {
                    return String::from_str(&env, "Stok produk tidak mencukupi");
                }
            }
        }

        return String::from_str(&env, "Produk tidak ditemukan");
    }

    // 4. DELETE: Fungsi untuk menghapus produk dari toko
    pub fn delete_product(env: Env, id: u64) -> String {
        // 1. Ambil data produk dari storage 
        let mut products: Vec<Product> = env.storage().instance().get(&SHOP_DATA).unwrap_or(Vec::new(&env));

        // 2. Cari index produk yang akan dihapus menggunakan perulangan
        for i in 0..products.len() {
            if products.get(i).unwrap().id == id {
                products.remove(i); // Hapus produk dari Vec

                // 3. Simpan perubahan ke storage
                env.storage().instance().set(&SHOP_DATA, &products);
                return String::from_str(&env, "Berhasil hapus produk");
            }
        }

        return String::from_str(&env, "Produk tidak ditemukan");
    }
}

mod test;