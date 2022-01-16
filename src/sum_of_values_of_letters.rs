use std::collections::HashMap;

pub fn sum_of_values() {
  let _letters:HashMap<&str, u32>= HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
    ("e", 5)
  ]);

  let string = "abcd".split("");

  // let mut sum: u32 = 0;
  // println!("{}", sum);
  for s in string {
    println!("{}", s);
  }
}