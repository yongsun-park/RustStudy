use std::thread;
use rayon::prelude::*;
use std::time::Duration;

fn main() {

    the_book_ex_16_1();
    // thread_for_example();
    // thread_fuctional_example();
    // rayon_example();

}

fn the_book_ex_16_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_for_example() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut threads = Vec::new();

    for i in list.into_iter() {
        // Create a new thread and move the ownership of `i` into the closure
        // passed to `thread::spawn`.
        threads.push(thread::spawn(move || {
            // Do something with the integer argument `i`.
            // 1초 대기 후, i*2를 리턴한다.
            std::thread::sleep(std::time::Duration::from_secs(1));
            i*2
        }));
    }

    // Wait for all threads to finish executing.
    for thread in threads {
        let result = thread.join().unwrap();
        println!("Thread finished with result: {}", result);
    }
}

fn thread_fuctional_example() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut threads = Vec::new();

    for i in list.into_iter() {
        // Create a new thread and move the ownership of `i` into the closure
        // passed to `thread::spawn`.
        threads.push(thread::spawn(move || {
            // Do something with the integer argument `i`.
            // 1초 대기 후, i*2를 리턴한다.
            std::thread::sleep(std::time::Duration::from_secs(1));
            i*2
        }));
    }

    // Wait for all threads to finish executing.
    let results = threads.into_iter()
        .map(|thread| thread.join().unwrap())
        .collect::<Vec<i32>>();

    println!("Results: {:?}", results);
}

// rayon dependency - not working concurrently
fn rayon_example() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = list.par_iter()
        .map(|&i| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            i*2
        })
        .collect::<Vec<i32>>();
    
    println!("Results: {:?}", results);
}