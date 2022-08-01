fn main() {
  let res1 = std::fs::read_to_string("file1.txt");
  match res1 {
    Ok(s) => println!("{}", s),
    Err(e) => println!("{}", e),
  }

  let res2 = std::fs::write("file1.txt", "New file 1");
  match res2 {
    Ok(_s) => println!("Wrote to file 1!"),
    Err(e) => println!("{}", e),
  }

  let res3 = std::fs::read_to_string("subdir/file2.txt");
  match res3 {
    Ok(s) => println!("{}", s),
    Err(e) => println!("{}", e),
  }

  let res4 = std::fs::write("subdir/file2.txt", "New file 2");
  match res4 {
    Ok(_s) => println!("Wrote to file 2!"),
    Err(e) => println!("{}", e),
  }
}
