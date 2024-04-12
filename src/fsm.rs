use std::collections::HashMap;
use std::hash::Hash;

pub trait DBObj {}

pub trait FSMController<T: FSMState> {
    fn save_func(&self, obj: &T, to_delete: Option<T::Action>, to_add: Option<&T::Action>);
}

pub trait ActionGetter<T: FSMState> {
    fn unroll(&self, state: &T) -> Vec<T::Action>;
}

pub trait StateEnum: Eq + PartialEq + Hash {}
pub trait EventEnum: Eq + PartialEq + Hash {}

pub trait FSMState: DBObj + Sized {
    type State: StateEnum;
    type Action: FSMAction;

    fn get_state(&self) -> Self::State;
}

pub trait FSMAction: DBObj + Sized {
    type Event: EventEnum;

    fn get_event(&self) -> Self::Event;
}

pub struct Path<S: FSMState> {
    pub source: S::State,
    pub destination: S::State,
    pub event: <<S as FSMState>::Action as FSMAction>::Event,
}

pub type HookFunc<S> = fn(state: &mut S, action: &<S as FSMState>::Action) -> Option<<S as FSMState>::Action>;
pub type Routes<S> = HashMap<<S as FSMState>::State, HookFunc<S>>;

pub struct FSM<T: FSMState, SD: FSMController<T>, AD: ActionGetter<T>> {
    state_driver: SD,
    action_driver: AD,
    paths: Vec<Path<T>>,
    routes: Routes<T>,
}

impl<'a, T: FSMState, SD: FSMController<T>, AD: ActionGetter<T>> FSM<T, SD, AD> {
    pub fn new(state_driver: SD, action_driver: AD, paths: Vec<Path<T>>, routes: Routes<T>) -> FSM<T, SD, AD> {
        FSM {
            state_driver,
            action_driver,
            paths,
            routes,
        }
    }

    pub fn run(&self, state: T) {
        let actions = self.action_driver.unroll(&state);

        let mut state = state;
        for action in actions {
            let hook = self.routes.get(&state.get_state()).unwrap();
            let new_action = hook(&mut state, &action);
            self.state_driver.save_func(&state, Some(action), new_action.as_ref());
        }
    }
}

