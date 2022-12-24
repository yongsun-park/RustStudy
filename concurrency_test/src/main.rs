use std::thread;

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut threads = Vec::new();

    for i in list.into_iter() {
        // Create a new thread and move the ownership of `i` into the closure
        // passed to `thread::spawn`.
        threads.push(thread::spawn(move || {
            // Do something with the integer argument `i`.
            // 3초 대기 후, i*2를 리턴한다.
            std::thread::sleep(std::time::Duration::from_secs(3));
            i*2
        }));
    }

    // Wait for all threads to finish executing.
    /*
    for thread in threads {
        let result = thread.join().unwrap();
        println!("Thread finished with result: {}", result);
    }
    */

    // Wait for all threads to finish executing.
    let results = threads.into_iter()
        .map(|thread| thread.join().unwrap())
        .collect::<Vec<i32>>();

    println!("Results: {:?}", results);
}