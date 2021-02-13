#[cfg(test)]
use crate::range_rng::RangeRng;

/// sample_gen_range_caller is used to exercise different implementations of the RangeRng
/// trait being passed to a function via trait reference.
#[cfg(test)]
pub fn sample_gen_range_caller<T: PartialOrd>(rng: &mut dyn RangeRng<T>, lower: T, upper: T) -> T {
    rng.gen_range(lower, upper)
}
