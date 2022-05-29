use super::SampleFactor;

#[derive(Clone, Copy)]
pub struct GridSamplerIter {
    i: i32,
    j: i32,
    n: i32,
    d: f64,
}

impl Iterator for GridSamplerIter {
    type Item = SampleFactor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i <= self.n {
            let u = self.i as f64 / self.d;
            let v = self.j as f64 / self.d;

            if self.j < self.n {
                self.j += 1;
            } else {
                self.i += 1;
                self.j = 1;
            }

            Some((u, v))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct GridSampler(pub i32);

impl IntoIterator for GridSampler {
    type Item = SampleFactor;

    type IntoIter = GridSamplerIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            i: 1,
            j: 1,
            n: self.0,
            d: (self.0 + 1) as f64,
        }
    }
}
