#![allow(unused)]


mod bird;


use std::io::Write;
use bird::BIRD;



fn csi(command: &str) {
    std::io::stdout().write(format!("\x1b[{}", command).as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
}
fn main() {
    // print!("\x1b[H");
    // print!("\x1b[2J");
    // println!();
    // Screen Size
    let height = 130;
    let width = 500;
    
    
    // for i in 1..=height {
        
    //     print!("█");
        
    //     for j in 2..width {
            
    //         if i == 1  {
    //             print!("▀");
    //         }
    //         else if i == height {
    //             print!("▄");
    //         }
    //         else {
    //             print!(" ");
    //         }
            
    //     }
    //     println!("█");
    // }
    
    // for i in 1..=height {
    //     csi("F");
    //     csi("2K");
    // }
    // print!("\x1b[H");
    // print!("\x1b[2J");
    // print!("\x1b[H\x1b[2J");
    // print!("\x1b[500K");
    // print!("\x1b[H");
    // print!("\x1b[0K");
    // std::io::stdout().flush();
    // print!("\x1b[J");

    // print!("Hello");
    // println!("Hello");
    // println!("Hello");
    // println!("Hello");
    // println!("Hello");
    // std::io::stdout().flush();

    // std::io::stdout().write("Hello".as_bytes()).unwrap();
    // std::io::stdout().write("\r".as_bytes()).unwrap();
    
    // csi("F");
    // csi("2K");
    // csi("2K");
    // csi("F");
    // csi("1G");
    // csi("1K");
    // csi("1G");
    // csi("2K");
    
    let lines = BIRD[0].lines().count();

    // csi("2J");

    while true {

        for Bird in BIRD.iter() {
            print!("{}", Bird);
            std::thread::sleep(std::time::Duration::from_millis(130));
                
            for i in 0..lines {
                csi("1G");
                csi("2K");
                
                if i != lines-1 {
                    csi("F");
                }
            }
        }

    }

    // println!();
    // println!();

    // println!();
}
