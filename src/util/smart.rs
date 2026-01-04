use std::fmt;

pub enum Mode {
    Round,
    Trunc,
}

pub struct SmartF64 {
    pub value: f64,
    pub precision: usize,
    pub mode: Mode,
}

impl fmt::Display for SmartF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let factor = 10f64.powi(self.precision as i32);
        let v = match self.mode {
            Mode::Round => (self.value * factor).round() / factor,
            Mode::Trunc => (self.value * factor).trunc() / factor,
        };

        // Write with precision, then trim *inside the formatter*
        let mut buf = ryu::Buffer::new();
        let mut s = buf.format_finite(v).to_string();

        if let Some(dot) = s.find('.') {
            let keep = dot + 1 + self.precision;
            s.truncate(s.len().min(keep));
            while s.ends_with('0') {
                s.pop();
            }
            if s.ends_with('.') {
                s.pop();
            }
        }

        f.write_str(&s)
    }
}
