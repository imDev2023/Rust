pub mod threads;
pub mod scopedthreads;
pub mod mutexes_threads;
pub mod test_mpsc;

fn main() {
    println!("Threading...");
    // threads::test_threads();
    // threads::spawn_thread();
    // scopedthreads::test_thread_variables();
    // mutexes_threads::test_mutex();
    test_mpsc::test_channels();
}
