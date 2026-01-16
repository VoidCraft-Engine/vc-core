// -----------------------------------------------------------------------------
// Config

pub const CHECK_CYCLE: u32 = 1 << 29;
pub const MAX_TICK_AGE: u32 = (u32::MAX / CHECK_CYCLE - 2) * CHECK_CYCLE - 1;

// -----------------------------------------------------------------------------
// Tick

use vc_reflect::derive::Reflect;

#[derive(Reflect, Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
#[reflect(mini, default, debug, hash, partial_eq)]
pub struct Tick {
    tick: u32,
}

impl Tick {
    /// The value of this is equal to [`MAX_TICK_AGE`].
    pub const MAX: Self = Self::new(MAX_TICK_AGE);

    #[inline(always)]
    pub const fn new(tick: u32) -> Self {
        Self { tick }
    }

    #[inline(always)]
    pub const fn get(self) -> u32 {
        self.tick
    }

    #[inline(always)]
    pub const fn set(&mut self, tick: u32) {
        self.tick = tick;
    }

    #[inline(always)]
    pub const fn relative_to(self, other: Self) -> Self {
        Self {
            tick: self.tick.wrapping_sub(other.tick),
        }
    }

    #[inline]
    pub const fn is_newer_than(self, other: Tick, now: Tick) -> bool {
        // TODO: used `Ord::min` instead if/when it's const stable.
        #[inline(always)]
        const fn min(x: u32, y: u32) -> u32 {
            if x < y { x } else { y }
        }

        let since_insert = min(now.relative_to(self).tick, MAX_TICK_AGE);
        let since_system = min(now.relative_to(other).tick, MAX_TICK_AGE);

        since_system > since_insert
    }

    #[inline]
    pub const fn check_age(&mut self, now: Tick) -> bool {
        let age = now.relative_to(*self);
        if age.get() > Tick::MAX.get() {
            *self = now.relative_to(Tick::MAX);
            true
        } else {
            false
        }
    }
}

// -----------------------------------------------------------------------------
// CheckTicks

#[derive(Debug, Clone, Copy)]
pub struct CheckTicks(pub(crate) Tick);

impl CheckTicks {
    /// Get the present `Tick` that other ticks get compared to.
    #[inline(always)]
    pub const fn tick(self) -> Tick {
        self.0
    }
}
