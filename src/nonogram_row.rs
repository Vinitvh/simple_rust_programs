pub fn nonogram_row() {
  let arr: [usize; 6] = [1,1,1,1,1,0];
  let _len = arr.len();

  let pos = arr.iter().position(|&x| x == 0);
  println!("{:?}", pos);
}