use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

   println!("GUESS THE NUMBER!");
   let secret_no = rand::thread_rng().gen_range(1..=10);
   
   loop{
       println!("Input your guess;-");

       let mut guess = String::new();

       io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line!");

       let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
       };

       println!("Your guess was {guess}");
       match guess.cmp(&secret_no) {
           Ordering::Less => println!("Too Small!"),
           Ordering::Greater => println!("Too Big!"),
           Ordering::Equal => {
                println!("You win!");
                break;
           }
       }
   }
}
