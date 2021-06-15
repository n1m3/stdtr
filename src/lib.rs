use stdtr_sys as ffi;

pub type Error = &'static str;

macro_rules! ensure {
    ($predicate:expr, $($tt:tt)*) => {
        if !$predicate {
            return Err($($tt)*);
        }
    }
}

/// Computes the integral from minus infinity to t of the Student
/// t distribution with integer k > 0 degrees of freedom
///
///
/// # Panics
///
/// if `k <= 0`
pub fn unchecked_stdr(k: i32, t: f64) -> f64 {
    checked_stdtr(k, t).unwrap()
}
/// Computes the integral from minus infinity to t of the Student
/// t distribution with integer k > 0 degrees of freedom
///
///
/// # Errors
///
/// if `k <= 0`
pub fn checked_stdtr(k: i32, t: f64) -> Result<f64, Error> {
    ensure!(k > 0, "k must be greater than 0.");
    Ok(unsafe { ffi::stdtr(k, t) })
}
/// Returns incomplete beta integral of the arguments, evaluated
/// from zero to x
///
/// # Panics
///
/// if `a <= 0` or `b <= 0` or `x < 0` or `x > 1`
pub fn unchecked_inc_beta(a: f64, b: f64, x: f64) -> f64 {
    checked_inc_beta(a, b, x).unwrap()
}
/// Returns incomplete beta integral of the arguments, evaluated
/// from zero to x
///
/// # Errors
///
/// if `a <= 0` or `b <= 0` or `x < 0` or `x > 1`
pub fn checked_inc_beta(a: f64, b: f64, x: f64) -> Result<f64, Error> {
    ensure!(a > 0. && b > 0., "a and b must be greater than 0.");
    ensure!(x >= 0., "x may not be smaller than 0.");
    ensure!(x <= 1.0, "x may not be greater than 1.");
    Ok(unsafe { ffi::incbet(a, b, x) })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_almost_equal(a: f64, b: f64) {
        if (a - b).abs() > 1.0E-14 {
            panic!("{:?} vs {:?}", a, b);
        }
    }
    #[test]
    fn test_inc_bet() {
        assert_eq!(unchecked_inc_beta(1.0, 2.0, 0.0), 0.0);
        assert_eq!(unchecked_inc_beta(1.0, 2.0, 1.0), 1.0);
        assert_almost_equal(unchecked_inc_beta(1.0, 2.0, 0.2), 0.36);
        assert_almost_equal(unchecked_inc_beta(5.0, 2.0, 0.5), 0.109375);
        assert_almost_equal(unchecked_inc_beta(1.0, 3.0, 0.6), 0.9359999999999999);
        assert_almost_equal(unchecked_inc_beta(4.0, 3.0, 0.6), 0.54432);
        assert_almost_equal(unchecked_inc_beta(2.0, 3.0, 0.5), 0.6875);
    }
}
