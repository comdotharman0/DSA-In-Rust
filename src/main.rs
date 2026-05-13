mod fibonacci;
mod sortings;
use sortings::bubble_sort;
fn experiment(){
let  mut a :[usize;4] = [9,1,3,6];
println!("{:#?}",a);
a[a.len()-1]=999;
println!("{:#?}",a);
}
fn main(){
bubble_sort();
//experiment();
//let fibo = fibonacci::Fibonacci::new();
//fibo.loops(18);
//fibo.for_loops(18);
//fibo.while_loops(18);
//println!("{}",fibo.recursion(18));
    println!("Hello, world!");
}
