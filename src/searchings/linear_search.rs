pub struct LinearSearch;

impl LinearSearch{
pub fn linear_search<const N:usize>(
a: [usize;N],
target_value: usize)->Option<usize>{
let mut result = None;
for i in 0..N{
if a[i]==target_value{
result=Some(i);
break;
}
}
result
}



}
