use std::fs::{read_to_string, write};

fn main() {
  read_to_string("file1.txt").expect("Could not read file1");
  write("file1.txt", "New file 1").expect("Could not write file1");
  read_to_string("file2.txt").expect("Could not read file2");
  write("file2.txt", "New file 2").expect("Could not write file2");
}
