mod structures;
mod exercises;
pub use crate::structures::using_vectors;
pub use crate::structures::using_strings;
pub use crate::structures::using_hashmaps;

fn main() {
    //println!("Vector: {:?} ", using_vectors::create_typed_variable());

    // using_vectors::reading_elems_from_vec();
    // let v: Vec<i32> = using_vectors::iterating_over_vec_vals();
    // println!("{:?}", v);

    // let mult_type_v = using_vectors::using_enum_store_mult();
    // println!("{:?}", mult_type_v);

    //string_main();
    //hashmap_main();
    call_exercises();

}



fn string_main() {
    // let myStr: String = using_strings::appending_to_string("Hello", "Fun".to_string());
    // println!("{:?}", myStr);

    // let s = using_strings::format_method("tic", "tac", "toe");
    // println!("{}",s);
    using_strings::iterate_over_string();
}

fn hashmap_main() {
    using_hashmaps::create_new_hashmap();
    using_hashmaps::accessing_values_in_hashmap();
    using_hashmaps::key_value_pairs();
    using_hashmaps::updating_value_based_on_old_value();
}

fn call_exercises() {
    let mut nums = vec![1, -2, 3, 4, 5];
    exercises::vector_statistics(&mut nums);
}