use std::fs;


fn print_text( data: & Vec<u8> ) {
    let mut text = String::from("");
    for k in data { 
        text.push( *k as char )
    }
    println!("{text}"); 
}

fn main() {

    let contents = fs::read_to_string("0059_plain_test_text.txt")
        .expect("Should have been able to read the file");

    let letters = contents.trim().split(",");
    let mut data = Vec::new(); 
    for lt in letters {
        let result = lt.parse::<u8>().unwrap();
        data.push(result);
        let temp: i32 = result.into();
        println!( "{temp}");
    }

    print_text(&data);

    //println!("The text: {contents}")
}
