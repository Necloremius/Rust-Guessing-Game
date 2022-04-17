use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
  
  println!("Welcome to the number guessing game! Guess the number between 1-100; you have 5 guesses.");
  
  let mut rng = rand::thread_rng();
  let target: i32 = rng.gen_range(1..101);
  let (mut guess, mut count): (i32, i32) = (0, 0);
  
  loop{
  
      if count==5{
          println!("Game over! The number was {}", target);
          break;
      }
      println!("Enter in a guess: ");
      
      let mut input:String = String::new();
      
      io::stdin()
        .read_line(&mut input)
        .expect("Please enter in a valid unsigned 32-bit integer.");
        
      let x: i32 = input.trim().parse().expect("Input not an integer");
      
      match x.cmp(&target) {
          Ordering::Greater => {
            println!("Your guess was GREATER than the target number.");
            count += 1;},
          Ordering::Less => {
            println!("Your guess was LESS than the target number.");
            count += 1;},
          Ordering::Equal => {
            println!("You guessed the number. It took you {} guesses.", count);
            break;},
      }
      
  }
}


