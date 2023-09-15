use std::{fs, ops::BitXor};

fn print_text(data: &Vec<u8>) {
    let mut text = String::from("");
    for k in data {
        text.push(*k as char)
    }
    println!("{text}");
}

fn apply_key(data: &mut Vec<u8>, key: &Vec<u8>) {
    for i in 0..data.len() {
        data[i] = data[i].bitxor(key[i % key.len()]);
    }
}



fn main() {
    let frequent_words = [ "the", "of", "and", "a", "to", "in", 
    "is", "you", "that", "it"  ];


    let contents = fs::read_to_string("0059_plain_test_text.txt")
        .expect("Should have been able to read the file");

    let letters = contents.trim().split(",");
    let mut data = Vec::new();
    for lt in letters {
        let result = lt.parse::<u8>().unwrap();
        data.push(result);
        let temp: i32 = result.into();
        println!("{temp}");
    }

    print_text(&data);

    // 141 = 'a' 
    // 172 = 'z' 
    let key = vec![120u8, 121u8, 123u8];

    apply_key(&mut data, &key);

    print_text(&data);

    apply_key(&mut data, &key);

    print_text(&data);

    //println!("The text: {contents}")
}
