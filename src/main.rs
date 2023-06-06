// fn main(){
//     let mut x : i32 = 50;
//     // println!("{}",x);
//     x=45;
//     println!("{}",x);
// }
/*extern crate rand;
use std :: io;
use std :: cmp :: Ordering;
use rand :: Rng;
fn main () {
println! ("Guess the number!");
let secret_number = rand :: thread_rng (). gen_range (1..101);
loop {
println! ("Please enter your number.");
let mut hunch = String :: new ();
io :: stdin (). read_line (& mut hunch)
.ok()
.expect ("Failed to read line");
let hunch: u32 = match hunch.trim (). parse () {
Ok (num) => num,
Err (_) => continue,
};
println! ("Your number was: {}", hunch);
match hunch.cmp (& secret_number) {
Ordering :: Less => println! ("Very small!"),
Ordering :: Greater => println! ("Very big!"),
Ordering :: Equal => {
println! ("You won! at");
break;
}
}
 }
 } */
// fn main(){
//     let my_number: u8 = 5;
//     println!("{}",my_number as char);
// }
// fn main (){
//     println!("{}", "rajan".len());
//     println!("{}", "बकमम".len());
//     println!("{}", "पजन".len());
// }
// fn main() {
//     let slice = "rajan";
//     println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
//     let slice2 = "र।जन";// name in nepali
//     println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
// }
// fn main(){
//     let my_number=1__0__8__9__u32;
//     println!("{}",my_number);
// }
// fn main(){
//     let my_number: u8 = 35 ;
//     let my_other_number = 45;

//     let third_number = my_number + my_other_number; 
//     // println!("{}",third_number );
//     println!("{}",third_number as char);
// }

// fn number()-> i32 {
//     8
// }
// fn main(){
//     println!("Hello, world number {}!",number());
// }
// fn main() {
//     println!("The smallest i8 is {} and the biggest i8 is {}.", std::i8::MIN, std::i8::MAX); // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
//     println!("The smallest u8 is {} and the biggest u8 is {}.", std::u8::MIN, std::u8::MAX);
//     println!("The smallest i16 is {} and the biggest i16 is {}.", std::i16::MIN, std::i16::MAX);
//     println!("The smallest u16 is {} and the biggest u16 is {}.", std::u16::MIN, std::u16::MAX);
//     println!("The smallest i32 is {} and the biggest i32 is {}.", std::i32::MIN, std::i32::MAX);
//     println!("The smallest u32 is {} and the biggest u32 is {}.", std::u32::MIN, std::u32::MAX);
//     println!("The smallest i64 is {} and the biggest i64 is {}.", std::i64::MIN, std::i64::MAX);
//     println!("The smallest u64 is {} and the biggest u64 is {}.", std::u64::MIN, std::u64::MAX);
//     println!("The smallest i128 is {} and the biggest i128 is {}.", std::i128::MIN, std::i128::MAX);
//     println!("The smallest u128 is {} and the biggest u128 is {}.", std::u128::MIN, std::u128::MAX);

// }
// fn main() {
//     let title = "TODAY'S NEWS";
//     println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
//     let bar = "|";
//     println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
//     let a = "SEOUL";
//     let b = "TOKYO";
//     println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
// }

// fn main() {
//     let name = "😂";
//     println!("My name is actually {}", name);
// }
/*fn multiply(number_one : i32 , number_two:i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}",number_one,number_two,result);
}
fn main (){
    multiply (8,9);
}
fn main(){
   let number_one = 6;
   let number_two = 8;
   println!("{} times {} is {}",number_one,number_two, number_one*&number_two);


 }*/

 /*fn main(){
     let my_number= {
         let second_number=30;
         second_number
     };
     println!("{}",my_number);
 }*/
  
//  fn main(){
//       let my_number=10 + 20;
//       println!("{}",my_number);
//  }


 /*fn main(){
     let mut number =54;
     println!("{}",number);{
      let number =56;
     println!("{}",number);}
     println!("{}",number);

 }*/



// fn main() {
//     let final_number = {
//         let y = 10;
//         let x = 9; // x starts at 9
//         let x = times_two(x); // shadow with new x: 18
//         let x = x + y; // shadow with new x: 28
//         x // return x: final_number is now the value of x
//     };
//     println!("The number is now: {}", final_number);
// }


// fn times_two(final_number: i64) -> i64 {
//             final_number * 2
// }



/*fn add(a:i32,b:i32) -> i32 {
    a+b
    
}
fn main(){
    let x=add(1,1);
    println!("the sum is {:?}",&x);
}*/

// fn main(){
//     let a=95;
//     if a>65 {
//         if a>150{
//             println!("Huge number");
//         }else if a>120{
//             println!("Big number");
//         }else{
//             println!("small number");
//     }
//     }
//}


//  fn  main(){
// let my_age=54;
//  if my_age>54{
//      println!("ok to purchase");
//  }else  if my_age<55 {
//      println!("cannot purchase");
//  }
// }


// fn main(){
//     let x:u8=54;
//     if x>50{
//         println!("rajan bhandari is virgin");
//     }else {
//         println!("rajan bhandari will always remain virgin");
//     }


// } 


fn main(){

}