// extern crate chrono;
use std::time::{Duration, Instant};
use chrono::prelude::*;

pub fn test_stdtime(){

    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(14500);
    let dur3 = dur1.checked_sub(dur2); 
    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();
    std::thread::sleep(Duration::from_millis(200));
    println!("Elapsed time: {}", now.elapsed().as_micros());
    println!("=====================");
}

pub fn test_chrono() {
    let utc = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("utc: {}", utc.format("%d %b %Y"));  // OutPut: day/MM/Year
    println!("local: {}", local);

    let date1 = NaiveDate::from_isoywd_opt(2025, 1, Weekday::Sun);
    let unwrapped_date = date1.unwrap();
    println!("{}", unwrapped_date.format("Day of the year is : %j"));
    
    unwrapped_date.iter_days().take(4).for_each(|d| println!("{}", d.format("%j")));
    
    let date2 = NaiveDate::from_yo_opt(2024, 366);
    println!("{}", date2.unwrap().format("%A %B %d"));

    let birthday= NaiveDate::parse_from_str("07||09|||2022", "%d||%m|||%Y");
    println!("{:#?}", birthday.err().unwrap());
    println!("{:#?}", birthday.ok().unwrap());




}