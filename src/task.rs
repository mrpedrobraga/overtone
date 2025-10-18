//! # Task
//!
//! A task is, much like a [`Future`], a representation of some concurrent computation
//! which may resolve in the future.
//!
//! `Task`, however, can give information about its progress while running.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context};

/// The main `Task` trait.
pub trait Task {
    /// The type of the value this task produces when complete.
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending
}

/// Trait for things that can be transformed into tasks.
pub trait IntoTask {
    type Task: Task;
    fn into_task(self) -> Self::Task;
}

pub struct FutureTask<Fut>(Fut);
impl<Fut> Task for FutureTask<Fut> where Fut: Future {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let future = unsafe{ Pin::map_unchecked_mut(self, |a| &mut a.0) };

        match future.poll(cx) {
            std::task::Poll::Ready(value) => {Poll::Ready(value)},
            std::task::Poll::Pending => {Poll::Pending}
        }
    }
}