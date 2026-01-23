pub mod gamemodes;
mod score;
mod tricks;

pub use gamemodes::*;
pub use score::Score;
pub use tricks::Tricks;


#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointsCoefficient {
    One = 1,
    Double = 2,
    DoubleNeg = -2,
}

impl PointsCoefficient {
    #[inline]
    #[must_use]
    pub const fn as_i8(self) -> i8 {
        self as i8
    }
}

impl From<PointsCoefficient> for i8 {
    #[inline]
    fn from(v: PointsCoefficient) -> Self {
        v as Self
    }
}

macro_rules! impl_from {
    ($self:ty: $($target:ty)+) => {
        $(
            impl From<$self> for $target {
                #[inline]
                fn from(v: $self) -> Self {
                    v as $target
                }
            }
        )*
    };
}
impl_from!(PointsCoefficient: i16 i32 i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::t;

    #[test]
    fn trivial_cases() {
        let a = t!(3);
        let b = t!(4);

        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum, t!(7));
        assert_eq!(sum.get(), 7);

        let sat_sum = a.saturating_add(b);
        assert_eq!(sum, sat_sum);

        let diff = b.checked_sub(a).unwrap();
        assert_eq!(diff, t!(1));

        let sat_diff = b.saturating_sub(a);
        assert_eq!(diff, sat_diff);
    }

    #[test]
    fn checked_add_with_u8_rhs_out_of_range_err() {
        let a = t!(5);

        let err = a.checked_add(14u8).unwrap_err();
        assert_eq!(err.0, 14);
    }

    #[test]
    fn checked_add_sum_out_of_range_err() {
        let a = t!(13);

        let err = a.checked_add(t!(13)).unwrap_err();
        assert_eq!(err.0, 26);
    }

    #[test]
    fn saturating_add_saturates() {
        let a = t!(13);
        let b = t!(11);

        let sum = a.saturating_add(b);
        assert_eq!(sum.get(), 13);
    }

    #[test]
    fn checked_sub_with_u8_rhs_out_of_range_err() {
        let a = t!(10);

        let err = a.checked_sub(14u8).unwrap_err();
        assert_eq!(err.0, 14);
    }

    #[test]
    fn checked_sub_underflow() {
        let a = t!(0);

        let err = a.checked_sub(t!(1)).unwrap_err();
        assert_eq!(err.0, 255);
    }

    #[test]
    fn saturating_sub_ok() {
        let a = t!(10);
        let b = t!(3);

        let diff = a.saturating_sub(b);
        assert_eq!(diff.get(), 7);
    }

    #[test]
    fn saturating_sub_saturates_at_zero() {
        let a = t!(2);
        let b = t!(13);

        let diff = a.saturating_sub(b);
        assert_eq!(diff.get(), 0);
        assert_eq!(diff, Tricks::MIN_TRICKS);
    }
}
