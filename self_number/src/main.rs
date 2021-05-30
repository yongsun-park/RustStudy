fn main() {
    let mut nums : [bool;10001] = [false; 10001];

    for i in 1..10000 {
        let next = i + sum_digits(i);
        if next <= 10000 {
            nums[next as usize] = true;
        }
    }

    for i in 1..10000 {
        if !nums[i] {
            println!("{}", i)
        }
    }
}

fn sum_digits(num : i32) -> i32 {
    get_digits(num).iter().sum()
}

fn get_digits(mut num : i32) -> Vec<i32>
{
    let mut result : Vec<i32> = Vec::new();
    
    while num > 0 {
        result.push(num%10);
        num = num / 10;
    }

    result
}

#[test]
fn get_digits_test() {
    assert_eq!(get_digits(1), vec![1]);
    assert_eq!(get_digits(12), vec![2, 1]);
    assert_eq!(get_digits(123), vec![3, 2, 1]);
    assert_eq!(get_digits(12345), vec![5, 4, 3, 2, 1]);
}