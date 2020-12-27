match number {
  // Match a single value
  1 => println!("One!"),
  // Match several values
  2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
  // Match an inclusive range
  13..=19 => println!("A teen"),
  // Handle the rest of cases
  _ => println!("Ain't special"),
}