use libc;

extern "C" {
    pub fn incbet(aa: libc::c_double, bb: libc::c_double, xx: libc::c_double) -> libc::c_double;
    pub fn stdtr(k: libc::c_int, t: libc::c_double) -> libc::c_double;
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
        assert_eq!(unsafe { incbet(1.0, 2.0, 0.0) }, 0.0);
        assert_eq!(unsafe { incbet(1.0, 2.0, 1.0) }, 1.0);
        assert_almost_equal(unsafe { incbet(1.0, 2.0, 0.2) }, 0.36);
        assert_almost_equal(unsafe { incbet(5.0, 2.0, 0.5) }, 0.109375);
        assert_almost_equal(unsafe { incbet(1.0, 3.0, 0.6) }, 0.9359999999999999);
        assert_almost_equal(unsafe { incbet(4.0, 3.0, 0.6) }, 0.54432);
        assert_almost_equal(unsafe { incbet(2.0, 3.0, 0.5) }, 0.6875);
    }
}
