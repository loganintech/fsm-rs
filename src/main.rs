#![allow(unused_imports)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::example::{led_fsm_wrapper, LEDController};
use crate::fsm::FSMController;
use crate::proto::button::{Button, ButtonEvent, LED, LEDState};

mod example;
mod proto;
mod fsm;
mod action_db;
mod state_db;


fn main() {
    let led_db: state_db::DB<LED> = state_db::DB::new();
    let button_db: action_db::DB<Button> = action_db::DB::new();
    let mut led_ref = Rc::new(RefCell::new(led_db));
    let mut button_ref = Rc::new(RefCell::new(button_db));


    let mut fsm = led_fsm_wrapper(led_ref.clone(), button_ref.clone());

    if let Some(led) = get_led(led_ref.clone()) {
        fsm.run(led.clone());
    }
    button_ref.borrow_mut().save(
        Button {
            event: ButtonEvent::BUTTON_EVENT_PRESSED.into(),
            ..Default::default()
        }
    );
    let led = get_led(led_ref.clone());
    if let Some(led) = led {
        fsm.run(led.clone());
    }
}

fn get_led(led_ref: Rc<RefCell<state_db::DB<LED>>>) -> Option<LED> {
    let mut led_borrow = led_ref.borrow_mut();
    led_borrow.save(
        LED {
            state: LEDState::LED_STATE_OFF.into(),
            ..Default::default()
        }
    );

    led_borrow.pop()
}