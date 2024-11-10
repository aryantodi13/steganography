//use std::io;
use std::fs::File;
use std::path::Path;
//use std::io::BufReader;
use std::io::prelude::*;
//use image::{DynamicImage, GenericImageView, io::Reader as ImageReader};
fn main() {

    // Read image
    //
    let img_path = Path::new("./assets/images/img.jpg");
    //let out_path = Path::new("./assets/images/output.jpg");

    let mut input_file = File::open(img_path).unwrap();
    //let inputReader = BufReader::new(inputFile);

    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer);
    println!("{}", buffer.len());
    for (i, j) in buffer.iter().enumerate(){
        println!("{}", j);
        if i == 5{ break};
    }


    /*
       let img = ImageReader::open("../assets/images/img.jpg")
       .expect("Failed to open image")
       .decode()
       .expect("Failed to decode image");

       println!("Loaded image dimensions {:?}", img.dimensions());
       let pixels = img.pixels();
       let mut iteratorCount = 0;
       for i in pixels {
       let rgba = i.2;
       println!("{:#?}", rgba.0[2]);
       iteratorCount+=1;
       if iteratorCount == 3 { break; }
       }

       let mut choice = Default::default();
       println!("***Menu***\n1. Add Password\n2. Retrieve password\n3. Retrieve all passwords\n(NOTE: image.png should be in the\nfolder or passed on as argument)");
       println!("Choice: ");

       io::stdin()
       .read_line(&mut choice)
       .expect("Failed to read line");


       let choice: u32 = match choice.trim().parse(){
       Ok(num) => num,
       Err(_) => {
       println!("Please enter numbers only");
       0
       }
       ,
       };
       match choice {
       1 => {
    // Add password
    println!("You Chose option 1")
    }
    2 => {
    // Add password
    println!("You Chose option 2")
    }
    3 => {
    // Add password
    println!("You Chose option 3")
    }
    4 => {
    // Add password
    println!("You Chose option 4")
    }
    _other => {
    println!("Not a valid option")
    }
    }
    */
}

