fn main() {
    // let mut s= String::from("Hello");
    // s.push_str(", World");
    // println!("{}", s); prints hello world


    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 ={}, s2={}", s1,s2); prints hello hello

    // ownershps
    let s3 = String::from("hello");//s3 comes into scope
    let x = 5; //x comes into scope
    takes_ownership(s3); // s3's value moves into the function
                                    // and so is no longer valid here
    makes_copy(x); //x would move into the function 
    //but it i32 is Copy, so it is okay to still use
    //x afterward 

    let s4 =String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {}.", s5,len);
}//x goes out of scope, but for s3 becasue its value has 
// has moved nothing special happens here.



// ownerships in functions
fn takes_ownership (some_string:String){
    // some_string comes into scope
    println!("{}", some_string);
}// some_string goes out of scope and drop method is called
// The backing memory is freed


fn makes_copy(some_integer:i32){
    // some_integer comes into scope
    println!("{}", some_integer);
}//here some_integer goes out of scope but nothing special happens

fn calculate_length(s:String)-> (String, usize){
    let length = s.len(); //len() returns the length of a String
     (s,length)
}