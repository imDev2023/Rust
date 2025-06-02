use std::collections::HashMap;

pub fn test_hashmap_basic() {
    let mut stock_list = HashMap::<String, f32>::new();
    println!("{}", stock_list.len());
    println!("{}", stock_list.is_empty());

    stock_list.insert("NVDA".to_string(), 478.52);
    stock_list.insert("AAPL".to_string(), 232.21);
    stock_list.insert("TSLA".to_string(), 350.08);

    stock_list.insert("AAPL".to_string(), 233.47);

    stock_list.entry("Meta".to_string()).or_insert(153.86);
    stock_list.entry("Meta".to_string()).or_insert(363.34);

    println!("{:#?}", stock_list);
    
    stock_list.remove(&("AAPL".to_string()));
    println!("{:#?}", stock_list);

    for (ticker, current_value) in stock_list{
        println!("{} is trading at {}", ticker, current_value);
    }
}