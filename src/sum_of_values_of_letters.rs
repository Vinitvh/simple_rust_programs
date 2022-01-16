use std::collections::HashMap;

pub fn sum_of_values() {
  let string = "excellent";

  let letters:HashMap<char, usize>= ('a'..='z').enumerate()
      .map(|tuple| (tuple.1, tuple.0 + 1)).collect();
  
  let sum: usize = string.to_lowercase().chars()
                .map(|ch| letters.get(&ch).unwrap_or(&0usize)).sum();
  println!("{}", sum);
}