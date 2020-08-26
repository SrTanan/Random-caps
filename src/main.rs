extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input); 
        let mut byte_array = input.as_bytes();
        
        let mut rng = rand::thread_rng();

        let mut output = String::new();
        for i in 0..byte_array.len() {
            if rng.gen::<f32>() < 0.5 {
                output += &String::from_utf8(vec![byte_array[i]]).ok().unwrap().to_lowercase();
                continue;
            }    

            output += &String::from_utf8(vec![byte_array[i]]).ok().unwrap().to_uppercase();
        }

        println!("{}",output);
    }
}