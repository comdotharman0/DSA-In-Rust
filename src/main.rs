mod fibonacci;
mod sortings;
use sortings::sortings_main;
fn experiment() {
    let mut a: [usize; 4] = [9, 1, 3, 6];
    println!("{:#?}", a);
    a[a.len() - 1] = 999;
    println!("{:#?}", a);
}
fn main() {
    sortings_main();
    //experiment();
    //let fibo = fibonacci::Fibonacci::new();
    //fibo.loops(18);
    //fibo.for_loops(18);
    //fibo.while_loops(18);
    //println!("{}",fibo.recursion(18));
    println!("Hello, world!");
}
