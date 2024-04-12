#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::example::led_fsm_wrapper;
use crate::proto::button::{LED, LEDState};

mod example;
mod proto;
mod fsm;


fn main() {
    let fsm = led_fsm_wrapper();
    fsm.run(LED { state: LEDState::LED_STATE_OFF.into(), ..Default::default() });
}
