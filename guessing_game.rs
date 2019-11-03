// Standard
use std::io;
use std::cmp::Ordering;
// Crate
use rand::Rng;

fn main() {
  println!("Guess the number!");

  // Using the crate rand associated/static methods to generate a random number
  let secret_number = rand::thread_rng().gen_range(1,101);

  loop {
    println!("Please input your guess.");

    /*
    Create a mutable variable named guess and using primitive 
    type String associated/static method called "new"
    */
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    
    // Changing type from String to Number. Called "Shadowing"
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    
    println!("You guessed: {}", guess);

    /* 
    Match expression will use the return value E.G Ordering:Less
    and try to match that value to an arm *Body. If Ordering::Greater
    was returned match will go arm by arm until it has "matched" Ordering::Greater
    it will execute the expression println!("Too Big!")
    */
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too Small!"),
      Ordering::Greater => println!("Too Big!"),
      Ordering::Equal => {
        println!("You Win!");
        break;
      }
    }
  }
}