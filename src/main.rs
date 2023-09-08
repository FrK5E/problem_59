use std::fs;

fn main() {

    let contents = fs::read_to_string("0059_cipher.txt")
        .expect("Should have been able to read the file");

    let letters = contents.split(",");
    for lt in letters { 
        println!( "{lt}");
    }
    //println!("The text: {contents}")
}
