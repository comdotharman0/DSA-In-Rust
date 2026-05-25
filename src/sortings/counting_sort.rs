use std::cmp::{Eq,PartialOrd};
pub struct CountingSort;

impl CountingSort{
pub fn counting_sort(mut a: Vec<usize>)->Vec<usize>
where
//T: Copy+Eq+PartialEq+PartialOrd+std::cmp::Ord
{
let N:usize= a.len();
let ZERO_LEN:usize =  *a.iter().max().unwrap();
let mut zero_a = vec![0;ZERO_LEN+1];

(0..N).for_each(|i|{
    zero_a[a[i]]+=1;
    //a.iter().filter(|&&j|{a[i]==j}).count();
});
a=vec![];
zero_a.iter().enumerate().for_each(|s|{
   // let mut small_a = vec![s.0;*s.1];
    //a.append(&mut small_a);
    (0..*s.1).for_each(|_|{
        a.push(s.0);
    });
    
});
    a
    
}

}


