fn main() {
  match std::fs::write("./file.out.txt", "This should not be written") {
    Ok(_) => println!("Write ok"),
    Err(e) => println!("{}", e),
  }
}
