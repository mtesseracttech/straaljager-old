use std::fmt;
use std::time::Duration;

pub struct CompoundTime {
    w: usize,
    d: usize,
    h: usize,
    m: usize,
    s: usize,
}

macro_rules! reduce {
    ($s: ident, $(($from: ident, $to: ident, $factor: expr)),+) => {{
        $(
            $s.$to += $s.$from / $factor;
            $s.$from %= $factor;
        )+
    }}
}

impl CompoundTime {
    #[inline]
    pub fn new(w: usize, d: usize, h: usize, m: usize, s: usize) -> Self {
        CompoundTime { w, d, h, m, s }
    }

    #[inline]
    pub fn balance(&mut self) {
        reduce!(self, (s, m, 60), (m, h, 60), (h, d, 24), (d, w, 7));
    }
}

impl fmt::Display for CompoundTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}w {}d {}h {}m {}s",
            self.w, self.d, self.h, self.m, self.s
        )
    }
}

pub fn duration_to_string(duration: &Duration) -> String {
    let milliseconds = ((duration.subsec_nanos() as f64 / 1.0e+9) * 1000.0) as usize;
    let mut compound = CompoundTime::new(0, 0, 0, 0, duration.as_secs() as usize);
    compound.balance();
    format!(
        "{}:{:0>2}:{:0>2}.{:0>3}",
        compound.w * 7 * 24 + compound.d * 24 + compound.h,
        compound.m,
        compound.s,
        milliseconds
    )
}
