pub trait State {
    fn input(&mut self, dt: &f32);
    fn update(&mut self, dt: &f32);
}
