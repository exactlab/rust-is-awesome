// a simple Point with a generic (template) type 
#[derive(Debug)]
struct Point<T>
{ 
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T 
  { &self.x }
}


// a simple String wrapper that it is not templated but for which we can define templated methods 
struct MyString 
{
  string : String
}

impl MyString
{  
  // println! requires that T implements Debug, 
  // this is called a trait-bound (just like concept in C++)
  fn f<T: std::fmt::Debug>(&self, point: Point<T>)
  { println!( "{:?}", point ); }
}


// an extension over Point that accepts 2 generic types 
// (they can also be the same type o course) 
struct Point2<X1, Y1> 
{
  x: X1,
  y: Y1,
}


impl<X1, Y1> Point2<X1, Y1> {
  // like for MyString, implementations (methods) can be templated on types that are
  // different from the ones Point2 is templated on 
  fn mixup<X2, Y2>(&self, other: &Point2<X2, Y2>) -> Point2<X1, Y2> 
    where X1 : Copy + std::fmt::Debug, Y2 : Copy    // an alternative way to specify trait-bounds,
  {                                                 // bounds can be combined with '+' 
    println!( "{:?}", self.x ); 
    Point2 {
      x: self.x,
      y: other.y,
    }
  }
}


fn main() 
{
  let p = Point { x: 5, y: 10 };
  println!( "p.x = {}", p.x() );
  println!( "p.y = {}", p.y );
    
  let ms = MyString{
    string: "pippo".to_string()
  };
    
  let p2 = Point { x: 2.5, y: 3.45 }; 
  ms.f( p );    // we call the same method with different types 
  ms.f( p2 );   // not polymorphic, the compiler will instatiate 2 versions for us 
    
  {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' }; // shadowing, no problem

    let p3 = p1.mixup( &p2 );
    println!( "p1.x = {}, p2.y = {}", p1.x, p2.y );
    println!( "p3.x = {}, p3.y = {}", p3.x, p3.y );
  }

  println!( "ms.string = {}", ms.string ); 
}
