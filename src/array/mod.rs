pub mod deque;
pub mod dual_array_deque;
pub mod queue;
pub mod rootish_array_stack;
pub mod stack;

pub fn allocate_heap<T: Default>(n: usize) -> Box<[T]> {
    std::iter::repeat_with(|| Default::default())
        .take(n)
        .collect::<Vec<T>>()
        .into_boxed_slice()
}
