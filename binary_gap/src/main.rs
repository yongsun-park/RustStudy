fn main() {
    
    for num in 1..128 {
        let binary = to_binary(num);
        let max_gap = max_gap(&binary);

        println!("{}: {} : {}", num, to_string_binary_vec(binary), max_gap);
    }
}

fn to_string_binary_vec(binary : Vec<i32>) -> String
{
    let mut result = String::new();
    for i in binary.iter() {
        result.push_str(&i.to_string());
    }

    result
}

fn to_binary(mut num : i32) -> Vec<i32>
{
    let mut result : Vec<i32> = Vec::new();
    while num > 0 {
        result.push(num % 2);
        num = num / 2;
    }

    result.reverse();
    result
}

fn max_gap(binary : &Vec<i32>) -> i32
{
    let mut max = 0;
    let mut cnt = 0;
    for i in binary.iter() {
        if i == &0 { 
            cnt += 1; 
        }
        else {
            if cnt > max {
                max = cnt;
            }
            cnt = 0;
        };
    }

    max
}