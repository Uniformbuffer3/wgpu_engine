#[derive(Clone)]
pub struct Requirements(wgpu::Features, wgpu::Limits);
impl Requirements {
    pub fn add(&mut self, requirements: (wgpu::Features, wgpu::Limits)) {
        self.0.insert(requirements.0);
        self.1 = self.1.clone().max(requirements.1);
    }
}

impl From<(wgpu::Features, wgpu::Limits)> for Requirements {
    fn from(requirements: (wgpu::Features, wgpu::Limits)) -> Self {
        Self(requirements.0, requirements.1)
    }
}
impl Into<(wgpu::Features, wgpu::Limits)> for Requirements {
    fn into(self) -> (wgpu::Features, wgpu::Limits) {
        (self.0, self.1)
    }
}
impl Default for Requirements {
    fn default() -> Self {
        Requirements(wgpu::Features::default(), wgpu::Limits::default())
    }
}
