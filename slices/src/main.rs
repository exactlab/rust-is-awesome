// sum a slice of i32
fn sum(slice: &[i32]) -> i32 
{
  let mut s = 0;
  for n in slice {  
    s += n; 
  }
  s
}


fn main() 
{   
  let array = [1, 2, 3, 4];     
  //let vector = vec![ 5, 6, 7, 8 ];
  let mut vector : Vec<i32> = Vec::with_capacity( 5 ); 
  vector.push(5); 
  vector.push(6); 
  vector.push(7);
  vector.push(8); 

  println!("array {:?}", array);
  println!("vector {:?}", vector);
  println!("sum 1 {}", sum(&array[1..3]));
  println!("sum 2 {}", sum(&array));
  println!("sum 3 {}", sum(&vector));
  println!("sum 4 {}", sum(&vector[0..2]));
}
