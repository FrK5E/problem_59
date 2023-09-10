use std::fs;

fn main() {

    let contents = fs::read_to_string("0059_cipher.txt")
        .expect("Should have been able to read the file");

    let letters = contents.split(",");
    let mut text = Vec::new(); 
    for lt in letters {
        let num = lt.parse::<u8>().unwrap(); 
        text.push(num);
        let temp: i32 = num.into();
        println!( "{temp}");
    }

    

    //println!("The text: {contents}")
}
