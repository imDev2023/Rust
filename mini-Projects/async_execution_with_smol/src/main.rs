#![allow(unused_variables)]

use futures::future::FutureExt;
use futures::{join, pin_mut, select};
fn main() {
    let num1 = num1().fuse();
    let num2 = num2().fuse();
    let num3 = num3().fuse();

    pin_mut!(num1, num2, num3);

    let result = smol::block_on(async {
        // join!(num1, num2, num3)
        loop {
            select! {
                x =num1 => println!("num1 is completed! {x}"),
                x =num2 => println!("num2 is completed! {x}"),
                x =num3 => println!("num3 is completed! {x}"),
                complete => {
                    println!("All futures have finished polling. Breaking out of loop");
                    break;
                }
            }
        }
    });
    println!("Final value is : {:?}", result);
}

async fn num1() -> u8 {
    return 8;
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(50));
    return 10;
}
async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(75));
    return 12;
}
