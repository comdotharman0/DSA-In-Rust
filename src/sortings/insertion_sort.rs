use std::cmp::{Eq,PartialOrd};
pub struct InsertionSort;

impl InsertionSort{
pub fn forloops<const N: usize,T>(mut a: [T;N])
->[T;N]
where
T: Copy+PartialEq+PartialOrd
{
for i in 1..N{
let mut insert_index= i;
let mut current_value=a[i];

for k in 1..=i{
let j=i-k;
if a[j]>current_value{
a[j+1]= a[j];
insert_index=j;
}else{
break;
}
}
a[insert_index]= current_value;

}

a
}

pub fn whileloops<const N: usize,T>(mut a: [T;N])
->[T;N]
where
T: Copy+PartialEq+PartialOrd
{
let mut i = 1;
while i<N{
let mut insert_index= i;
let mut current_value=a[i];
let mut k=1;
while k<=i{
let j=i-k;
if a[j]>current_value{
a[j+1]= a[j];
insert_index=j;
}else{
break;
}
k+=1;
}
a[insert_index]= current_value;
i+=1;
}

a
}

pub fn loops<const N: usize,T>(mut a :[T;N])
->[T;N]
where
T: Copy+PartialEq+PartialOrd
{let mut i =1;
loop{
if i==N{
break;
}
let mut insert_index= i;
let mut current_value=a[i];
let mut k=1;
loop{
if k>i{
break;
}
let j= i-k;
if a[j]>current_value{
a[j+1]= a[j];
insert_index=j;
}else{
break;
}
k+=1;
}
a[insert_index]= current_value;
i+=1;
}

a
}

pub fn recursions<const N: usize, T>(
mut a: [T; N], i: usize, k: usize) -> [T; N]
where
    T: Copy + PartialEq + PartialOrd,
 {
if i<N{
if k<=i{
    let j=i-k;
    if a[j]>a[j+1]{
    let temp = a[j+1];
        a[j+1]=a[j];
        a[j]= temp;
        a=Self::recursions(a,i,k+1);
    }else{
        a=Self::recursions(a,i+1,1);
    }
}else{
  a=Self::recursions(a,i+1,1);
}
}
 a
}

pub fn iterators<const N: usize, T>(
mut a: [T; N]) -> [T; N]
where
    T: Copy + PartialEq + PartialOrd,
 {
(1..N).for_each(|i|{
let mut insert_index=i;
let current_value=a[i];
(1..=i).for_each(|k|{
let j=i-k;
if a[j]>current_value{
a[j+1]=a[j];
insert_index=j;
}else{}
});
a[insert_index]=current_value;
});
a
}

}


