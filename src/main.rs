use std::fs;

fn read_content_by_line() {
    let contents = fs::read_to_string("simples/simple-text.txt")
        .expect("Should read the contents of the file");

    println!("contents: {}", contents);
}

fn main() {
   read_content_by_line(); 
}
