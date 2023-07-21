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
        UNSPEC,
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
                UNSPEC => {
                    panic!("called transition() on unspecified state")
                }
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
    state:   MachineState,
}

impl Processor {
    fn new(_control: ()) -> Self {
        Self {
            control: _control,
            state:   MachineState::INIT,
        }
    }
}

impl Fiber for Processor {
    type Output = ();
    type Yield<'a> = &'a MachineState
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Output> {
        let mut state = MachineState::transition(
            mem::replace(&mut self.state, MachineState::UNSPEC),
            &self.control,
        );
        mem::swap(&mut self.state, &mut state);

        match self.state {
            MachineState::INIT => panic!("already processed"),
            MachineState::Raw(_) => State::Yield(&self.state),
            MachineState::Proc(_) => State::Yield(&self.state),
            MachineState::DONE(_) => State::Done(()),
            MachineState::UNSPEC => panic!("unspecified state"),
        }
    }
}
