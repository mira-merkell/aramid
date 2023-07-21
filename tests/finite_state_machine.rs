//! Finite-state machine
//!
//! TODO: Turn this into a tutorial.

mod fsm {
    pub struct RawData;
    pub struct ProcData {
        params: u8,
    }

    fn recv() -> RawData {
        RawData {}
    }

    fn process(
        _data: RawData,
        params: u8,
    ) -> ProcData {
        ProcData {
            params,
        }
    }

    fn send(data: ProcData) -> u8 {
        data.params
    }

    pub enum MachineState {
        INIT,
        Raw(RawData),
        Proc(ProcData),
        DONE(u8),
    }
    use MachineState::*;

    impl Default for MachineState {
        fn default() -> Self {
            Self::INIT
        }
    }

    pub fn transition(
        state: MachineState,
        control: &u8,
    ) -> MachineState {
        match state {
            INIT => Raw(recv()),
            Raw(data) => Proc(process(data, *control)),
            Proc(data) => DONE(send(data)),
            DONE(x) => DONE(x),
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
    control: u8,
    state:   Option<MachineState>,
}

impl Processor {
    fn new(control: u8) -> Self {
        Self {
            control,
            state: Some(Default::default()),
        }
    }
}

impl Fiber for Processor {
    type Return = ();
    type Yield<'a> = &'a mut u8
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Return> {
        let mut state =
            Some(fsm::transition(self.state.take().unwrap(), &self.control));
        mem::swap(&mut self.state, &mut state);

        match &mut self.state {
            Some(MachineState::INIT) => panic!("already processed"),
            Some(MachineState::DONE(_)) => State::Done(()),
            Some(_) => State::Yield(&mut self.control),
            None => panic!(),
        }
    }
}
