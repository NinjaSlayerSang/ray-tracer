use ray_tracer::sampler::{GridSampler, RandomSampler};

#[test]
fn test() {
    println!("random sampler:");
    for (u, v) in RandomSampler(10) {
        println!("({}, {})", u, v)
    }

    println!("grid sampler:");
    for (u, v) in GridSampler(4) {
        println!("({}, {})", u, v)
    }
}
