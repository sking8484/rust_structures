pub use std::collections::HashMap;

pub fn create_new_hashmap() -> HashMap<String, i32> {
  let mut scores = HashMap::new();

  scores.insert("Blue".to_string(), 10);
  scores.insert("Yellow".to_string(), 50);

  println!(
    "{:?}", scores
  );

  scores
}



pub fn accessing_values_in_hashmap() {
  let scores: HashMap<String, i32> = create_new_hashmap();

  let team_name = String::from("Bluee");
  let score = scores.get(&team_name);
  match score {
    None => println!("Does Not Exist"),
    Some(scr) => println!("Score: {}", scr)
  }
}

pub fn key_value_pairs() {
  let scores: HashMap<String, i32> = create_new_hashmap();

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}


pub fn updating_value_based_on_old_value() {
  let mut map = HashMap::new();
  let text = "hello world wonderful world";

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map)


}
