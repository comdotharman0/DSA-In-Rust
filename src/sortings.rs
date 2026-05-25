mod bubble_sort;
mod selection_sort;
mod insertion_sort;
mod quick_sort;
mod counting_sort;
use bubble_sort::BubbleSort;
use selection_sort::SelectionSort;
use insertion_sort::InsertionSort;
use quick_sort::QuickSort;
use counting_sort::CountingSort;
pub fn bubble_sort() {
 println!("Bubble Sort");   
let bsl = BubbleSort::loops(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    let bsfl = BubbleSort::forloops(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let bswl = BubbleSort::whileloops(
[5, 9, 3, 1, 2]);
    let bsr = BubbleSort::recursions(
[5, 9, 3, 1, 2], 0, 0);
println!(
"Loop {:?} \n ForLoop {:?} \n WhileLoop {:?} Recursions {:?}",
        bsl, bsfl, bswl, bsr
    );

    let ssfl = SelectionSort::forloops([5, 9, 3, 1, 2]);

    println!("ForLoop {:?}", ssfl);
}


pub fn selection_sort() {
println!("Selection Sort");
let bsl = SelectionSort::loops(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    let bsfl = SelectionSort::forloops(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let bswl = SelectionSort::whileloops(
[5, 9, 3, 1, 2]);
    let bsr = SelectionSort::recursions(
[5, 9, 3, 1, 2], 0, 1);
println!(
"Loop {:?} \n ForLoop {:?} \n WhileLoop {:?} Recursions {:?}",
        bsl, bsfl, bswl, bsr
    );
}

pub fn insertion_sort() {
println!("Insertion Sort");
let bsl = InsertionSort::loops(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    let bsfl = InsertionSort::forloops(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let bswl = InsertionSort::whileloops(
[5, 9, 3, 1, 2]);
   let bsr = InsertionSort::recursions(
[10,9,8,7,6,5,4,3,2,1,0], 1, 1);
let isi= InsertionSort::iterators(
[4, 2, 5, 3]);
println!(
"Loop {:?} \n ForLoop {:?} \n WhileLoop {:?} Recursions {:?}",
        bsl, bsfl, bswl, bsr
    );
println!("Iterators {:?}",isi);
}

pub fn quick_sort() {
println!("Quick Sort");
let bsl = QuickSort::quicksort(
[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],0, None);
println!("Simple {:?}",bsl);
}

pub fn counting_sort() {
println!("Counting Sort");
let bsl = CountingSort::counting_sort(
vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
println!("Simple {:?}",bsl);
}
pub fn sortings_main(){
bubble_sort();
selection_sort();
insertion_sort();
quick_sort();
counting_sort();

}
