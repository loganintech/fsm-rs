use std::any::Any;
use std::collections::HashMap;
use protobuf::EnumOrUnknown;
use crate::proto::button::*;
use crate::fsm::*;
use crate::proto;


// region: Theoretically generated by protogen

impl StateEnum for LEDState {}

impl EventEnum for ButtonEvent {}

impl DBObj for LED {}

impl DBObj for Button {}

impl FSMState for LED {
    type State = LEDState;
    type Action = Button;

    fn get_state(&self) -> Self::State {
        self.state()
    }
}


impl FSMAction for Button {
    type Event = ButtonEvent;

    fn get_event(&self) -> Self::Event {
        self.event()
    }
}


pub struct LEDController {
    // Pretend DB Connection
    db_state: LED
}

impl ActionGetter<LED> for ButtonController {
    fn unroll(&self, state: &LED) -> Vec<Button> {
        vec![Button {
            event: self.db_state.event.into(),
            special_fields: Default::default(),
        }]
    }
}

impl FSMController<LED> for LEDController {
    fn save_func(&self, obj: &LED, to_delete: Option<Button>, to_add: Option<&Button>) {
        println!("Save function");
    }
}

pub struct ButtonController {
    // Pretend DB Connection
    db_state: Button
}

pub fn led_fsm_wrapper() -> FSM<LED, LEDController, ButtonController> {
    let paths = [
        Path {
            source: LEDState::LED_STATE_ON,
            destination: LEDState::LED_STATE_OFF,
            event: ButtonEvent::BUTTON_EVENT_PRESSED,
        },
        Path {
            source: LEDState::LED_STATE_OFF,
            destination: LEDState::LED_STATE_ON,
            event: ButtonEvent::BUTTON_EVENT_PRESSED,
        },
    ];

    let mut routes: Routes<LED> = HashMap::new();
    routes.insert(LEDState::LED_STATE_ON, |state: &mut LED, action: &Button| -> Option<Button> {
        println!("On Handler");
        None
    });
    routes.insert(LEDState::LED_STATE_OFF, |state: &mut LED, action: &Button| -> Option<Button>  {
        println!("Off Handler");
        None
    });

    FSM::new(LEDController {
        db_state: LED {
            state: LEDState::LED_STATE_OFF.into(),
            special_fields: Default::default(),
        }
    }, ButtonController {
        db_state: Button {
            event: ButtonEvent::BUTTON_EVENT_PRESSED.into(),
            special_fields: Default::default(),
        }
    }, paths.into(), routes)
}
