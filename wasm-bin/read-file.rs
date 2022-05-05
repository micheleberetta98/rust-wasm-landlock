fn main() {
  let content = std::fs::read_to_string("hello.txt");
  match content {
    Ok(s) => println!("{}", s),
    Err(e) => println!("{}", e),
  }
}
