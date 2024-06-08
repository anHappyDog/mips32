use crate::cp0::{self,sr, ST_IE, ST_IM7};
use crate::Reg;
pub const TIME_INTERVAL: usize = 2000000;
pub fn enable_interrupt() {
    sr::write(sr::read() | ST_IE);
}

pub fn disable_timer_interrupt() {
    sr::write(sr::read() & !ST_IM7);
}

pub fn enable_timer_interrupt() {
    cp0::count::write(0);
    cp0::compare::write(TIME_INTERVAL);
    sr::write(sr::read() | ST_IM7);
}
