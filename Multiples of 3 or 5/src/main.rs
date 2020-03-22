fn solution(num: i32) -> i32 {
    // time to code
    let mut sum = 0;
    let i:i32;
    for i in 1..num
    {
        if i % 5 == 0 || i % 3 == 0
        {
            sum = sum + i;
        }
        
    }
    println!("{}", sum);
    return sum;
  }

fn main() {
    solution(10); // test case 1
    solution(11); // test case 2
    solution(6);  // test case 3
}
