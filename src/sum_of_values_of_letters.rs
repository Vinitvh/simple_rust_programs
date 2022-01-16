use std::collections::HashMap;

pub fn sum_of_values() {
  let letters:HashMap<&str, u32>= HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
    ("e", 5)
  ]);

  for (letter, _val) in letters {
    print!("{}", letter);
  }
}