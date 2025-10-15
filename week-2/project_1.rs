<<<<<<< HEAD
fn main() {
   let p:f64 = 520_000_000.0;
   let r:f64 = 10.0;
   let n:f64 = 5.0;

   //compound interest
    let a = p * (1.0+(r/100.0)).powf(n);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("compound interest is {}", ci);
}
=======
fn main() {
   let p:f64 = 520_000_000.0;
   let r:f64 = 10.0;
   let n:f64 = 5.0;

   //compound interest
    let a = p * (1.0+(r/100.0)).powf(n);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("compound interest is {}", ci);
}
>>>>>>> d9e09c769ff333635bcb5689b26daf073e587d33
