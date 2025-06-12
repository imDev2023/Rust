use std::ops::Add;
struct GroceryItem {
    name: String,
    price: f32,
}

struct GroceryBill {
    items: Vec<GroceryItem>,
    tax_rate: f32,
}

impl GroceryBill {
    fn calculate_total(&self) -> f32 {
        let items_total = self.items.iter().fold(0f32, |a, i| return a + i.price);

        let tax_value = items_total * self.tax_rate;

        return items_total + tax_value;
    }
}

impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;
    fn add(self, rhs: GroceryItem) -> Self::Output {
        let mut bill = self;
        bill.items.push(rhs);
        return bill;
    }
}

fn main() {
    let mut new_bill = GroceryBill {
        items: Vec::new(),
        tax_rate: 0.027,
    };

    let carrots = GroceryItem {
        name: "Bag of Carrots 1 KG".to_string(),
        price: 1.99,
    };

    let cheese = GroceryItem {
        name: "Cottage Cheese 500 gram.".to_string(),
        price: 3.5,
    };

    new_bill = new_bill + carrots + cheese;
    let total = new_bill.calculate_total();

    println!(" The total of your grocery bill is {}", total);
}
