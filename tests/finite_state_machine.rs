//! Finite-state machine
//!
//! TODO: Turn this into a tutorial.

mod fsm {
    pub struct RawData;
    pub struct ProcData;

    fn recv() -> RawData {
        RawData {}
    }

    fn process(_data: RawData) -> ProcData {
        ProcData {}
    }

    fn send(_data: ProcData) {}

    pub enum MachineState {
        INIT,
        Raw(RawData),
        Proc(ProcData),
        DONE(()),
    }
    use MachineState::*;

    impl MachineState {
        pub fn transition(
            self,
            _control: &(),
        ) -> Self {
            match self {
                INIT => Raw(recv()),
                Raw(data) => Proc(process(data)),
                Proc(data) => {
                    send(data);
                    DONE(())
                }
                DONE(()) => DONE(()),
            }
        }
    }
}

use std::mem;

use aramid::{
    Fiber,
    State,
};
use fsm::MachineState;
struct Processor {
    control: (),
    state:   Option<MachineState>,
}

impl Processor {
    fn new(_control: ()) -> Self {
        Self {
            control: _control,
            state:   Some(MachineState::INIT),
        }
    }
}

impl Fiber for Processor {
    type Return = ();
    type Yield<'a> = &'a MachineState
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Return> {
        let mut state = Some(MachineState::transition(
            self.state.take().unwrap(),
            &self.control,
        ));
        mem::swap(&mut self.state, &mut state);

        match &self.state {
            Some(MachineState::INIT) => panic!("already processed"),
            Some(MachineState::DONE(_)) => State::Done(()),
            Some(state) => State::Yield(state),
            None => panic!(),
        }
    }
}
