pub use std::collections::HashMap;


pub fn vector_statistics(nums: &mut Vec<i32>) {

  let mut occurences: HashMap<i32, i32> = HashMap::new();

  nums.sort();
  let mut median = 0.0;


  if nums.len() % 2 != 0 {
    let middle_index = (nums.len() as f32 / 2.0).floor() as usize;
    median = nums[middle_index] as f32;
  } else {
    let middle_index1 = (nums.len() as f32 / 2.0).floor() as usize;
    let middle_index2 = ((nums.len() as f32 / 2.0).floor() as i32 + 1) as usize;
    median = (nums[middle_index1] as f32 + nums[middle_index2] as f32)/2.0;
  }

  println!("The median is: {}", median);


  for i in nums {
    let count = occurences.entry(*i).or_insert(0);
    *count += 1;
  }

  println!("The occurences are: {:?}", occurences);

}