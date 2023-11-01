use std::fmt;
use std::time::{Duration, Instant};

pub struct HumanDuration(Duration);
impl HumanDuration {
    pub fn since(i: Instant) -> Self {
        Self(Instant::now().duration_since(i))
    }

    /// Get this duration as an f64 representing the duration in fractional seconds
    #[inline]
    pub fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }
}
impl From<Duration> for HumanDuration {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}
impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.0.as_secs();
        let alt = f.alternate();
        macro_rules! try_unit {
            ($secs:expr, $sg:expr, $pl:expr, $s:expr) => {
                let cnt = t / $secs;
                if cnt == 1 {
                    if alt {
                        return write!(f, "{}{}", cnt, $s);
                    } else {
                        return write!(f, "{} {}", cnt, $sg);
                    }
                } else if cnt > 1 {
                    if alt {
                        return write!(f, "{}{}", cnt, $s);
                    } else {
                        return write!(f, "{} {}", cnt, $pl);
                    }
                }
            };
        }

        if t > 0 {
            try_unit!(365 * 24 * 60 * 60, "year", "years", "y");
            try_unit!(7 * 24 * 60 * 60, "week", "weeks", "w");
            try_unit!(24 * 60 * 60, "day", "days", "d");
            try_unit!(60 * 60, "hour", "hours", "h");
            try_unit!(60, "minute", "minutes", "m");
            try_unit!(1, "second", "seconds", "s");
        } else {
            // Time was too precise for the standard path, use millis
            let t = self.0.as_millis();
            if t > 0 {
                return write!(f, "{}{}", t, if alt { "ms" } else { " milliseconds" });
            }
        }
        write!(f, "0{}", if alt { "s" } else { " seconds" })
    }
}
