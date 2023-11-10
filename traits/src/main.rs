use core::fmt::Debug;

pub trait Summary {
    fn summarize<T: Debug>(&self, obj: &T) -> String
    { 
        format!("default {:?}", obj) 
    }
}

pub trait Summary2 {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
}

impl NewsArticle {
    fn summarize<T>(&self, _ :&T) -> String {
        format!("struct")
    }
}

impl Summary for NewsArticle {
    fn summarize<T>(&self, _: &T) -> String {
        format!("trait")
    }
}


pub struct Tweet {

}

impl Summary for Tweet {
}

impl Summary2 for Tweet {
    fn summarize(&self) -> String {
        format!("tweet 2")
    }
}


fn function<T: Summary>(input: &T) 
{
  println!( "{:?}", input.summarize( &"turbo cipolla" ) );
  // expands to Summary::summarize( &input, "..." )
}


use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn function(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn main()
{
  let news = NewsArticle{};
  let tweet = Tweet{}; 
  
  function( &news );
  function( &tweet ); 
  
  
  let p = Pair{ x: 2, y: 4 };
  p.function(); 
}
