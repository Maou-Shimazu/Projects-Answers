fn main() {
  let mut nums: Vec<i32> = vec![11];
  loop {
    let i = nums.len() - 1;
    let num = nums[i];
    if num == 1 {
      break;
    }
    let odd = num % 2 == 1;
    if odd {
      nums.push(3 * num + 1);
    } else {
      nums.push(num / 2);
    }
  }
  println!("{:?}", nums)
}
    
