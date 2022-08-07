fn main() {
  for i in 1..51 {
    if i %3 ==0 || i%10==3 {
      println!("A");
      continue;
    } else if i > 30 && i < 40 {
      println!("A");
      continue;
    }
    else {
      println!("{}", i);
    }
  }
}