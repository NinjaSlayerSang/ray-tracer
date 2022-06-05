mod grid_sampler;
mod random_sampler;

pub use {grid_sampler::GridSampler, random_sampler::RandomSampler};

pub type SampleFactor = (f64, f64);
