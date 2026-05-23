use std::cmp::{Eq, PartialOrd};
pub struct SelectionSort;

impl SelectionSort {
    pub fn forloops<const N: usize, T>(mut a: [T; N])
 -> [T; N]
    where
        T: Copy + PartialOrd,
    {
        for i in 0..N - 1 {
            let mut min_index = i;
            for j in (i + 1)..N {
                if a[j] < a[min_index] {
                    min_index = j;
                }
            }
            
(a[min_index], a[i]) = (a[i], a[min_index]);
        }
        a
    }

pub fn whileloops<const N: usize, T>(mut a: [T; N])
 -> [T; N]
    where
        T: Copy + PartialOrd,
    {
let mut i=0;
while i<N - 1 {
            let mut min_index = i;
let mut j=i+1;
            while j<N {
                if a[j] < a[min_index] {
                    min_index = j;
                }
j+=1;
            }

(a[min_index], a[i]) = (a[i], a[min_index]);
i+=1;      
  }

a

}

pub fn loops<const N: usize, T>(mut a: [T; N])
 -> [T; N]
    where
        T: Copy + PartialOrd,
    {
let mut i = 0;
loop{
if i==N-1{
break;
}   
        let mut min_index = i;
let mut j= i+1;
            loop {
if j==N{
break;
}
                if a[j] < a[min_index] {
                    min_index = j;
                }
j+= 1;
            }

(a[min_index], a[i]) = (a[i], a[min_index]);
i+=1; 
       }

a

}

pub fn recursions<const N: usize, T>(mut a: [T; N], mut i: usize,mut j:usize)
 -> [T; N]
    where
        T: Copy + PartialOrd,
    {
 if i < N - 1{
let mut min_index=i;
            if j < N {
                if a[j] < a[min_index] {
min_index=j;
               }
(a[min_index], a[i]) = (a[i], a[min_index]);
                a = Self::recursions(a, i, j+1);
            } else {
               
                i += 1;
                a = Self::recursions(a, i, i);
            }
        }
a

}


}
