use std::ops::Range;

pub fn overlaps<T: PartialOrd>(r1: &Range<T>, r2: &Range<T>) -> bool {
    r1.start < r2.end && r2.start < r1.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        assert_eq!(overlaps(&(1..2), &(2..2)), false);
        assert_eq!(overlaps(&(1..5), &(2..8)), true);
    }
}
