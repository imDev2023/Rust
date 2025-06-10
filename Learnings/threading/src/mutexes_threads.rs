use std::{ops::AddAssign, sync::{Arc, Mutex} };
use std::thread::scope;

pub fn test_mutex() {
    let score = Arc::new(Mutex::new(0u16));

    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);
    // println!("{:?}", data);
    // drop(data);

//     let myfunc = move || {
//         let mut data =score.lock().unwrap();
//         for i in 1..10 {
//             data.add_assign(i);
//         }
//     };

//    _= scope(|scope:|{
//          scope.spawn(myfunc);
//     });

    scope(|s|{
        let score_clone = Arc::clone(&score);
        s.spawn(move || {
            let mut data = score_clone.lock().unwrap();
            for i in 1..10 {
                data.add_assign(i);
                println!("Thread 2 is adding {i}");
            }
        });
    });
    println!("{}", score.lock().unwrap());
}