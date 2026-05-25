use std::cmp::{Eq,PartialOrd};
pub struct RadixSort;

impl RadixSort{
pub fn radix_sort(mut a:Vec<usize>)->Vec<usize>
where
//T:Copy+Eq+PartialEq+PartialOrd+Ord+std::ops::Div<isize>
{
let mut radix_a=vec![vec![];10];
let max_val = *a.iter().max().unwrap();
let mut exp=1;
//println!("{:?}",radix_a);
while max_val/exp>0{
    for val in a.drain(..){
        
        let radix_index= (val/exp)%10;
        radix_a[radix_index].push(val);
    }
    
    for bucket in &mut radix_a{
        for val in bucket.drain(..){
          
            a.push(val);
            }
    }
     exp*=10   
    }

    a
}



}


