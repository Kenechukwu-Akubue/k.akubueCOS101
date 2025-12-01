struct Brand {
    hp:u64,
    ibm:u64,
    toshiba:u64,
    dell:u64,
}

impl Brand {
    fn total_price(&self, qty:u64) -> u64 {
        qty*(self.hp + self.ibm + self.toshiba + self.dell)
        }
    }

fn main() {
    let unit_price = Brand {
        hp: 650_000,
        ibm: 755_000,
        toshiba: 550_000,
        dell: 850_000,
    };
    let qty:u64 = 3;
    let total = unit_price.total_price(qty);
    println!("Unit cost: HP Laptops - {}, IBM Laptops - {}, Toshiba Laptops - {}, Dell Laptops - {}", unit_price.hp, unit_price.ibm, unit_price.toshiba, unit_price.dell);
    println!("Quantity: {}", qty);
    println!("Total HP Laptop cost: {}", unit_price.hp * qty);
    println!("Total IBM Laptop cost: {}", unit_price.ibm * qty);
    println!("Total Toshiba Laptop cost: {}", unit_price.toshiba * qty);
    println!("Total Dell Laptop cost: {}", unit_price.dell * qty);
    println!("Total cost: {}", total);
}
