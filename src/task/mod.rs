use futures_signals::signal::Mutable;

/// A DiscreteTask is an abstraction of a procedure that might not have been completed yet (a [`Future`]) and is being completed partially.
/// The procedure will be completed at some moment, yielding a single result.
pub struct DiscreteTask<T> {
    pub step_count: Mutable<u32>,
    pub current_step_idx: Mutable<u32>,
    pub progress: Mutable<f32>,
    pub result: Option<T>
}

impl<T> DiscreteTask<T> {
    fn new(step_count: u32) -> Self {
        Self {
            step_count: Mutable::new(step_count),
            current_step_idx: Mutable::new(0),
            progress: Mutable::new(0.0),
            result: None
        }
    }

    fn get_result(&self) -> &Option<T> {
        &self.result
    }
}