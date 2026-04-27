pub trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

pub struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            for j in 0..n - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

pub struct QuickSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        data.sort_unstable();
    }
}
