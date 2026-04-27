use crate::strategy::SortStrategy;

/// Static dispatch — the strategy type is a generic parameter.
/// The compiler monomorphises a separate `Sorter` for each concrete `S`,
/// so method calls are direct with no runtime overhead.
pub struct Sorter<S: SortStrategy> {
    strategy: S,
}

impl<S: SortStrategy> Sorter<S> {
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    pub fn run(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
}
