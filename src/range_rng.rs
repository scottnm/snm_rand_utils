use rand::Rng;

pub trait RangeRng<T: PartialOrd> {
    fn gen_range(&mut self, lower: T, upper: T) -> T;
}

pub fn select_rand<'a, T>(seq: &'a [T], rng: &mut dyn RangeRng<usize>) -> &'a T {
    let index = rng.gen_range(0, seq.len());
    &seq[index]
}

pub struct ThreadRangeRng {
    rng: rand::rngs::ThreadRng,
}

impl ThreadRangeRng {
    pub fn new() -> ThreadRangeRng {
        ThreadRangeRng {
            rng: rand::thread_rng(),
        }
    }
}

impl<T: PartialOrd + rand::distributions::uniform::SampleUniform> RangeRng<T> for ThreadRangeRng {
    fn gen_range(&mut self, lower: T, upper: T) -> T {
        self.rng.gen_range(lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;
    use crate::test_utils::sample_gen_range_caller;

    #[test]
    fn test_thread_random() {
        // this test is mostly here to verify that things compile
        let mut rng = ThreadRangeRng::new();
        let first_value = rng.gen_range(0, 10);
        let next_value = sample_gen_range_caller(&mut rng, 10, 20);
        assert_ne!(first_value, next_value);
    }

    #[test]
    fn test_select_rand() {
        let mut rng = mocks::SingleValueRangeRng::new(2usize);
        let elements = ["a", "b", "c", "d"];
        assert_eq!(select_rand(&elements, &mut rng), &"c");
    }
}
