pub fn nonogram_row() {
  let arr: [i8; 13] = [1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1];
  let mut last_element = false;
  let mut count = 0;
  let mut count_vec = Vec::new();

  for element in arr.iter() {
      if *element == 1 {
          count += 1;
          last_element = true;
      } else if last_element == true && *element == 0 {
        count_vec.push(count);
          last_element = false;
          count = 0;
      }
  }

  if count != 0 {
    count_vec.push(count);
  }

  for element in count_vec.iter() {
      println!("{}", element);
  }
}