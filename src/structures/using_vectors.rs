pub fn create_typed_variable() -> Vec<i32> {
  let v: Vec<i32> = Vec::new();
  return v;
}


pub fn create_new_vec() -> Vec<i32> {
  let v = vec![1, 2, 3];
  v
}

pub fn updating_a_vector() -> Vec<i32> {
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);

  v
}

pub fn reading_elems_from_vec() {
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  let third: Option<&i32> = v.get(2);

  match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element"),
  }
}

pub fn iterating_over_vec_vals() -> Vec<i32> {
  let v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }

  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
  }

  v
}

#[derive(Debug)]
pub enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn using_enum_store_mult() -> Vec<SpreadsheetCell> {
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  row
}
