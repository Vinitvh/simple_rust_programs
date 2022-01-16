use std::collections::HashMap;

pub fn sum_of_values() {
  let letters:HashMap<&str, u32>= HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
    ("e", 5)
  ]);

  let mut sum: u32 = 0;
  for val in letters.values() {
    sum = sum + val;
  }
  println!("{}", sum);
}