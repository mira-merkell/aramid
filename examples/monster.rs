//! A monster, patrolling its dungeon, walking back and forth 🕹️👾.
//!
//! The idea for this example was taken from Bob Nystrom's
//! [blog post on fibers][bob-nystrom-fibers].
//!
//! [bob-nystrom-fibers]: (https://journal.stuffwithstuff.com/2010/07/13/fibers-coroutines-in-finch/).

use aramid::{
    Fiber,
    State,
};

fn main() {
    // Create a monster ---
    let mut monster = Monster::default();

    // Start walking ---
    let motion = Walk::new(&mut monster);

    // Take 3 steps ---
    let motion = (0..3).fold(motion, |mut m, i| {
        println!("Position: {}. Take a step ({i})", m.get());
        m.run().unwrap()
    });
    println!("--- Take a break");

    // Monster is busy walking ---
    // We cannot access the monster's state directly, since it's borrowed by
    // Walk. This line won't compile:
    //
    // let pos = monster.position;

    // Take 2 more steps ---
    let more = (0..2).fold(motion, |mut m, i| {
        println!("Position: {}. Take a step ({i})", m.get());
        m.run().unwrap()
    });

    // If we try to take another step,
    // the fiber will switch its state to `Wait`:
    let state = more.run();
    assert!(state.is_done());

    // Monster is waiting ---
    let wait = state.unwrap_done();
    let state = wait.complete(|_| println!("waiting..."));

    // We're done waiting, get back to walking
    let mut count = 0;
    let _ = state
        .complete(|fbr| {
            println!("Position: {}. Take a step ({count})", fbr.get());
            count += 1;
        })
        .complete(|_| println!("waiting..."))
        .complete(|_| println!("walking..."));

    // We've dropped the fiber here.
    // We can now modify monster again, i.e.
    monster.walk_right = false;
}

struct Monster {
    wait_frames: u64,
    walk_steps:  u64,
    walk_right:  bool,
    position:    i64,
}

impl Default for Monster {
    fn default() -> Self {
        Self {
            wait_frames: 3,
            walk_steps:  5,
            walk_right:  true,
            position:    0,
        }
    }
}

/// Wait for a specified number of frames
struct Wait<'a> {
    monster: &'a mut Monster,
    elapsed: u64,
}

impl<'a> Wait<'a> {
    fn new(monster: &'a mut Monster) -> Self {
        Self {
            monster,
            elapsed: 0,
        }
    }
}

impl<'a> Fiber for Wait<'a> {
    type Output = Walk<'a>;
    type Yield = u64;

    fn run(mut self) -> State<Self> {
        if self.elapsed < self.monster.wait_frames {
            self.elapsed += 1;
            State::Yield(self)
        } else {
            // State transition
            State::Done(Walk::new(self.monster))
        }
    }

    fn get(&mut self) -> Self::Yield {
        self.elapsed
    }
}

/// Walk for a specified number of steps
struct Walk<'a> {
    monster: &'a mut Monster,
    steps:   u64,
}

impl<'a> Walk<'a> {
    fn new(monster: &'a mut Monster) -> Self {
        Self {
            monster,
            steps: 0,
        }
    }
}

impl<'a> Fiber for Walk<'a> {
    type Output = Wait<'a>;
    type Yield = i64;

    fn run(mut self) -> aramid::State<Self> {
        if self.steps < self.monster.walk_steps {
            self.steps += 1;
            self.monster.position +=
                if self.monster.walk_right { 1 } else { -1 };
            State::Yield(self)
        } else {
            // Next time, walk in the opposite direction
            self.monster.walk_right = !self.monster.walk_right;
            // State transition
            State::Done(Wait::new(self.monster))
        }
    }

    fn get(&mut self) -> Self::Yield {
        self.monster.position
    }
}
