pub trait Lock {
    fn lock(&mut self);
    fn unlock(&mut self);
}
