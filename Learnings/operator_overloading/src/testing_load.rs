use std::ops::Add;
struct GroceryItem {
    name: String,
    price: f32
}

struct GroceryBill {
    items : Vec<GroceryItem>,
    tax_rate: f32,
}


impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;
    fn add(self, rhs:GroceryItem)-> Self::Output {
        let mut bill = self;
        bill.items.push(rhs);
        return bill
    }
}

impl GroceryBill {
    fn calculate_total(&self) -> f32{
        todo!()
    }
}

pub fn main_testing(){

    let new_bill = GroceryBill{
        items: Vec::new(),
        tax_rate: 2.7,
    };

    let carrots = GroceryItem{
        name: "Bag of carrots".to_string(),
        price: 1.99,
    };

    let cheese = GroceryItem{
        name:"500 gram of cheese".to_string(),
        price: 5.99,
    };


}