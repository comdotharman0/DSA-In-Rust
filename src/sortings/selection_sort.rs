use std::cmp::{Eq,PartialOrd};
pub struct SelectionSort;

impl SelectionSort{
pub fn forloops<const N:usize,T>(mut a:[T;N])
->[T;N]
where 
T:Copy+PartialOrd
{

for i in 0..N-1{

let mut min_index = i;
for j in (i+1)..N{
if a[j]<a[min_index]{
min_index = j;

}
}
(a[min_index],a[i]) = (a[i],a[min_index]);
}
a

}



}
