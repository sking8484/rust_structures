pub fn creating_new_string() {
  let s = String::new();

  let data = "initial contents";
  let s = data.to_string();

  let s = "initial contents".to_string();

  let s = String::from("initial contents");
}

pub fn appending_to_string(string: &str, string_type: String) -> String {
  let mut s = String::from("foo");
  s.push_str(string);
  let s1 = s + &string_type;
  println!("{}", s1);

  s1
}

pub fn format_method(str1: &str, str2: &str, str3: &str) -> String {
  let s = format!("{}-{}-{}", str1.to_string(), str2.to_string(), str3.to_string());
  s
}

pub fn iterate_over_string() {
  for c in "3Å".chars() {
    println!("{}", c);
  }


  for c in "abcd".bytes() {
    println!("{}",c);
  }
}

