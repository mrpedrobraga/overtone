pub trait PushReturn<T> {
    fn push_and_get(&mut self, t: T) -> &mut T;
}

impl<T> PushReturn<T> for Vec<T> {
    fn push_and_get(&mut self, t: T) -> &mut T {
        self.push(t);
        self.last_mut().unwrap()
    }
}