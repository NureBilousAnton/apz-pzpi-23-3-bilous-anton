use crate::strategy::SortStrategy;

/// Dynamic dispatch — the strategy is a trait object stored on the heap.
/// The concrete type is resolved at runtime through a vtable, which allows
/// swapping the strategy while the program is running.
pub struct DynSorter {
    strategy: Box<dyn SortStrategy>,
}

impl DynSorter {
    pub fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
        self.strategy = strategy;
    }

    pub fn run(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
}
