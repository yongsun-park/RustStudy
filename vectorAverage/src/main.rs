fn main()
{
    let v = vec![ 1, 2, 3, 4 ];
    
    let sum = sum(&v);
    let avg = avg(&v);
    
    println!("sum: {}, average: {}", sum, avg);
}

fn sum(v: &Vec<i32>) -> i32
{
    let mut sum = 0;
    for i in v.iter() {
        sum = sum + i;
    }
    
    sum
}

fn avg(v : &Vec<i32>) -> f32
{
    let sum = sum(&v);
    let avg = sum / v.len() as i32;
    avg as f32
} 
