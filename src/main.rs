use std::io;
fn main() {
    let mut choice = Default::default();
    println!("***Menu***\n1. Add Password\n2. Retrieve password\n3. Retrieve all passwords\n(NOTE: image.png should be in the\nfolder or passed on as argument)");
    println!("Choice: ");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Read image

    /*
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

