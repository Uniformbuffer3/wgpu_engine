#[derive(Clone)]
pub struct Requirements(crate::wgpu::Features, crate::wgpu::Limits);
impl Requirements {
    pub fn add(&mut self, requirements: (crate::wgpu::Features, crate::wgpu::Limits)) {
        self.0.insert(requirements.0);
        self.1 = self.1.clone().max(requirements.1);
    }
}

impl From<(crate::wgpu::Features, crate::wgpu::Limits)> for Requirements {
    fn from(requirements: (crate::wgpu::Features, crate::wgpu::Limits)) -> Self {
        Self(requirements.0, requirements.1)
    }
}
impl Into<(crate::wgpu::Features, crate::wgpu::Limits)> for Requirements {
    fn into(self) -> (crate::wgpu::Features, crate::wgpu::Limits) {
        (self.0, self.1)
    }
}
impl Default for Requirements {
    fn default() -> Self {
        Requirements(
            crate::wgpu::Features::default(),
            crate::wgpu::Limits::default(),
        )
    }
}
