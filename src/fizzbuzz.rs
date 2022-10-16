pub fn fizzbuzz_1() {
  println!("This fizzbuzz is [1]");

  let mut x = 1;

  while x <= 100 {
    if x % 15 == 0 {
      println!("Fizzbuzz! : {}", x)
    } else if x % 3 == 0 {
      println!("Fizz! : {}", x)
    } else if x % 5 == 0 {
      println!("Buzz! : {}", x)
    }
    x += 1;
  }
}

pub fn fizzbuzz_2() {
  println!("This fizzbuzz is [2]");

  for x in 1..=100 {
    if x % 15 == 0 {
      println!("Fizzbuzz! : {}", x)
    } else if x % 3 == 0 {
      println!("Fizz! : {}", x)
    } else if x % 5 == 0 {
      println!("Buzz! : {}", x)
    }
  }
}

pub fn fizzbuzz_3() {
  for x in 1..=100 {
    match x % 15 {
      0 => println!("Fizzbuzz! : {}", x),
      3 | 6 | 9 | 12 => println!("Fizz! : {}", x),
      5 | 10 => println!("Buzz! : {}", x),
      _ => println!("{}", x)
    }
  }
}

pub fn fizzbuzz_4() {
  for x in 1..=100 {
    match x {
      e if e % 15 == 0 => println!("Fizzbuzz! : {}", e),
      e if e % 5 == 0 => println!("Fizz! : {}", e),
      e if e % 3 == 0 => println!("Buzz! : {}", e),
      e => println!("{}", e)
    }
  }
}
