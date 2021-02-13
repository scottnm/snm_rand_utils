use crate::range_rng::RangeRng;

pub struct SingleValueRangeRng<T: PartialOrd + Copy> {
    value: T,
}

pub struct SequenceRangeRng<T: PartialOrd + Copy> {
    next: usize,
    seq: Vec<T>,
}

impl<T: PartialOrd + Copy> SingleValueRangeRng<T> {
    pub fn new(value: T) -> SingleValueRangeRng<T> {
        SingleValueRangeRng { value }
    }
}

impl<T: PartialOrd + Copy> RangeRng<T> for SingleValueRangeRng<T> {
    fn gen_range(&mut self, lower: T, upper: T) -> T {
        assert!(lower <= self.value);
        assert!(upper > self.value);
        self.value
    }
}

impl<T: PartialOrd + Copy> SequenceRangeRng<T> {
    pub fn new(value: &[T]) -> SequenceRangeRng<T> {
        SequenceRangeRng {
            next: 0,
            seq: Vec::from(value),
        }
    }
}

impl<T: PartialOrd + Copy> RangeRng<T> for SequenceRangeRng<T> {
    fn gen_range(&mut self, lower: T, upper: T) -> T {
        let value = self.seq[self.next];
        self.next = (self.next + 1) % self.seq.len();

        assert!(lower <= value);
        assert!(upper > value);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::sample_gen_range_caller;

    #[test]
    fn test_single_value_random() {
        let mut rng = SingleValueRangeRng::new(10i32);
        let first_value = rng.gen_range(0, 100);
        for _ in 1..10 {
            let next_value = sample_gen_range_caller(&mut rng, 0, 100);
            assert_eq!(first_value, next_value);
        }
    }

    #[test]
    fn test_select_rand() {
        let mut rng = SingleValueRangeRng::new(10i32);
        let first_value = rng.gen_range(0, 100);
        for _ in 1..10 {
            let next_value = sample_gen_range_caller(&mut rng, 0, 100);
            assert_eq!(first_value, next_value);
        }
    }
}
