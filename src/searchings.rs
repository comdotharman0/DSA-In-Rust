mod linear_search;
mod binary_search;
use linear_search::LinearSearch;
use binary_search::BinarySearch;

pub fn searchings_main(){
let lsa = LinearSearch::linear_search(
[6,7,8,4,3,2,1,0,20,10],4);
let bsa = BinarySearch::binary_search(
[2],1);
println!(
"Linear Search = {:#?}, Binary Search = {:#?}",
lsa, bsa);
}
