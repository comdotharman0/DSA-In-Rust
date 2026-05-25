pub struct BinarySearch;

impl BinarySearch{
pub fn binary_search<const N:usize>(
mut a: [usize;N],
target_value:usize)->Option<usize>{
let mut result = None;
let mut left = 0;
let mut right= N-1;
while left<=right{
let mid = (left+right)/2;
if a[mid]==target_value{
result = Some(mid);
return result;
}else if a[mid]>target_value{
if right==0{
return None;
}
right = mid - 1;

}else{
if left==N-1{
return None;
}
left = mid + 1;
}
println!("Index = {}",mid);
}

result
}


}

