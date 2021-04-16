use std::io;

fn main() {
    
    // input
    println!("Please input n");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num : usize = input.trim().parse().expect("Fail to parse num");

    println!("input num : {}", num);

    println!("{} : {}", num, fibo0(num));
    println!("{} : {}", num, fibo1(num));
    println!("{} : {}", num, fibo2(num));
}

fn fibo0(num: usize) -> u32
{
    // early return
    match num {
        v if v < 2 => return v as u32,
        _ => num,
    };

    let mut pre_1 = 0;
    let mut pre_2 = 1;
    let mut n_result : u32 = 0;

    for n in (2..num+1) {
        n_result = pre_1 + pre_2;
        // println!("{} : {}", n, n_result);

        pre_1 = pre_2;
        pre_2 = n_result;
    }

    n_result
}

fn fibo1(n : usize) -> u32
{
    if n < 2 {
        n as u32
    }
    else {
        fibo1(n-1) + fibo1(n-2)
    }
}

fn fibo2(num: usize) -> u32
{
    let mut _cache = vec![0; num+1];
    _cache[1] = 1;

    for n in (2..num+1) {
        _cache[n] = _cache[n-1] + _cache[n-2];
        // println!("{} : {}", n, _cache[n]);
    }

    _cache[num]
}
