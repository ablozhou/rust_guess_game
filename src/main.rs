use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let guess_num = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("please guess a number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error input");
        println!("you guest {}",guess);
        let guess:u32 = match guess.trim().parse() {
            Ok(num) =>num, 
            Err(_)=> {
                println!("error input");
                continue;
            }
        };
        match guess.cmp(&guess_num){
            
            Ordering::Less => println!(" too small!"),
            Ordering::Greater => println!(" too big!"),
            Ordering::Equal=> {
                println!(" you win!!");
                break;
            }

        }
    }

}
