use std::cmp::{Eq,PartialOrd};
pub struct QuickSort;

impl QuickSort{
pub fn partition<const N:usize,T>(mut a:[T;N],low:usize,high:usize)->([T;N],usize)
where T: Copy+ Eq+PartialOrd+PartialEq{
    let pivot = a[high];
    let mut i = low;
    for j in low..high{
        if a[j]<=pivot{
           
            (a[i],a[j])= (a[j],a[i]);
            i+=1;
        }
    }
    (a[i],a[high])= (a[high],a[i]);
    (a,i)
}

pub fn quicksort<const N:usize,T>(mut a:[T;N],low:usize,high:Option<usize>)->[T;N]
where T: Copy+ Eq+PartialOrd+PartialEq{
let high = high.unwrap_or(N-1);
if  low<high{

  let   pivot_index;
  (a,pivot_index)= Self::partition(a, low, high);
  if pivot_index>0{
       a= Self::quicksort(a, low, Some(pivot_index-1));
       }
       a= Self::quicksort(a, pivot_index+1, Some(high));
     
}
a
}

}


