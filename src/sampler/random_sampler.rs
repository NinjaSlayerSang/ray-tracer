use rand::{thread_rng, Rng};

use super::SampleFactor;

#[derive(Clone, Copy)]
pub struct RandomSamplerIter {
    n: i32,
}

impl Iterator for RandomSamplerIter {
    type Item = SampleFactor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            self.n -= 1;
            Some(thread_rng().gen::<(f64, f64)>())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct RandomSampler(pub i32);

impl IntoIterator for RandomSampler {
    type Item = SampleFactor;

    type IntoIter = RandomSamplerIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { n: self.0 }
    }
}
