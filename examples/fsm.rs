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
        Init,
        Raw(RawData),
        Proc(ProcData),
        Done(u8),
    }
    use MachineState::*;

    impl Default for MachineState {
        fn default() -> Self {
            Self::Init
        }
    }

    pub fn transition(
        state: MachineState,
        control: &u8,
    ) -> MachineState {
        match state {
            Init => Raw(recv()),
            Raw(data) => Proc(process(data, *control)),
            Proc(data) => Done(send(data)),
            Done(x) => Done(x),
        }
    }
}

use std::mem;

use aramid::{
    Fiber,
    State,
};
use fsm::MachineState;
pub struct Processor {
    control: u8,
    state:   Option<MachineState>,
}

impl Processor {
    pub fn new(control: u8) -> Self {
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
            Some(MachineState::Init) => panic!("already processed"),
            Some(MachineState::Done(_)) => State::Done(()),
            Some(_) => State::Yield(&mut self.control),
            None => panic!(),
        }
    }
}

struct ConveyorBelt {
    control: u8,
    proc:    Vec<Processor>,
}

impl ConveyorBelt {
    fn _add_new(&mut self) {
        self.proc.push(Processor::new(self.control))
    }
}

impl Fiber for ConveyorBelt {
    type Return = ();
    type Yield<'a> = ()
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Return> {
        if let Some(mut proc) = self.proc.pop() {
            if let State::Yield(yld) = proc.run() {
                *yld = self.control;
                self.proc.insert(0, proc);
            }
            State::Yield(())
        } else {
            State::Done(())
        }
    }
}

fn main() {}
