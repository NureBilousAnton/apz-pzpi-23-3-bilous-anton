mod strategy;
mod sorter;
mod dyn_sorter;

use strategy::{BubbleSort, QuickSort};
use sorter::Sorter;
use dyn_sorter::DynSorter;

fn main() {
    // Static dispatch
    let mut data = vec![5, 2, 8, 1, 9, 3];
    let sorter = Sorter::new(BubbleSort);
    sorter.run(&mut data);
    println!("{:?}", data); // [1, 2, 3, 5, 8, 9]

    // Dynamic dispatch with runtime strategy swap
    let mut data = vec![5, 2, 8, 1, 9, 3];
    let mut sorter = DynSorter::new(Box::new(BubbleSort));
    sorter.run(&mut data);

    data = vec![7, 3, 6, 1, 4];
    sorter.set_strategy(Box::new(QuickSort));
    sorter.run(&mut data);
    println!("{:?}", data); // [1, 3, 4, 6, 7]
}
