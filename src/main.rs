use std::{fs, ops::BitXor};

fn convert_to_string(data: &Vec<u8>, result: &mut String) {
    result.clear();
    for k in data {
        result.push(*k as char)
    }
}

fn apply_key(data: &mut Vec<u8>, key: &Vec<u8>) {
    for i in 0..data.len() {
        data[i] = data[i].bitxor(key[i % key.len()]);
    }
}

fn detect_english(data: &Vec<u8>) -> bool {
    let mut string1 = String::with_capacity(data.len());
    convert_to_string(&data, &mut string1);

    let number_of_spaces = string1.matches(" ").count();
    if (data.len() as f64)/(number_of_spaces as f64) > 8.0 { 
        return false;
    }


    let frequent_words = [
        "the", "of", "and", "a", "to", "in", "is", "you", "that", "it",
    ];
    
    
    let mut found = 0i32;  
    for word in frequent_words {
        if string1.contains(word) {
            found += 1;
        }
    }

    if ( found >= 8 ) { 
        println!("Found {found} words.\n {string1}");
        return true;
    }

    return false;
}

fn main() {
    let contents =
        fs::read_to_string("0059_cipher.txt").expect("Should have been able to read the file");

    let letters = contents.trim().split(",");
    let mut data = Vec::new();
    for lt in letters {
        let result = lt.parse::<u8>().unwrap();
        data.push(result);
    }

    let mut counter = 1u32;
    for first_char in 97u8..122u8+1u8 {
        for second_char in 97u8..122u8+1u8 {
            for third_char in 97u8..122u8+1u8 {
                let key = vec![first_char, second_char, third_char];
                apply_key(&mut data, &key);
                if detect_english(&data) {
                    println!("Heureka ");
                    return;
                }
                counter +=1;
                println!( "{counter}" ); 
                apply_key(&mut data, &key);
            }
        }
    }
}
