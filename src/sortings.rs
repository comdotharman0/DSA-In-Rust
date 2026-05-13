mod bubble_sort;
mod selection_sort;
use bubble_sort::BubbleSort;
use selection_sort::SelectionSort;
pub fn bubble_sort(){

let bsl = BubbleSort::loops(
[10,9,8,7,6,5,4,3,2,1,0]
);

let bsfl = BubbleSort::forloops(
[10,9,8,7,6,5,4,3,2,1,0]
);
let bswl = BubbleSort::whileloops(
[5,9,3,1,2]
);
let bsr = BubbleSort::recursions(
[5,9,3,1,2],0,0);
println!(
"Loop {:?} \n ForLoop {:?} \n WhileLoop {:?} Recursions {:?}",
bsl, bsfl, bswl,bsr);

let ssfl = SelectionSort::forloops(
[5,9,3,1,2]
);

println!("ForLoop {:?}",ssfl);

}
