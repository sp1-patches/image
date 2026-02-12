use num_traits::{NumCast, ToPrimitive};

use crate::Primitive;

pub(crate) struct LazyConstrastLut<S: Primitive> {
    cache: [Option<S>; 256],
    max: f32,
    percent: f32,
}

impl<S: Primitive + ToPrimitive + 'static> LazyConstrastLut<S> {
    pub(crate) fn new(contrast: f32) -> Self {
        let max: f32 = NumCast::from(S::DEFAULT_MAX_VALUE).unwrap();
        let percent = ((100.0 + contrast) / 100.0).powi(2);
        Self {
            cache: [None; 256],
            max,
            percent,
        }
    }

    pub(crate) fn get(&mut self, b: S) -> S {
        let idx = b.to_usize().unwrap();
        if let Some(v) = self.cache[idx] {
            return v;
        }
        let c: f32 = NumCast::from(b).unwrap();
        let d = ((c / self.max - 0.5) * self.percent + 0.5) * self.max;
        let e = d.clamp(0.0, self.max);
        let result: S = NumCast::from(e).unwrap();
        self.cache[idx] = Some(result);
        result
    }
}
