//! A legion of monsters, patrolling a dungeon, walking back and forth 👾🕹️.
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
    // Create a legion ---
    let mut legion = Legion::new(5);

    for i in 0..1000 {
        let mosters = legion.run().unwrap();
        for m in mosters {
            println!(
                "Round: {i}, monster id: {}, position: {}",
                m.id, m.position
            );
        }
    }
}

struct Legion {
    stack: Vec<Monster>,
}

impl Legion {
    fn new(n: u64) -> Self {
        assert!(n > 0);
        let mut stack = Vec::new();
        for i in 0..n {
            let monster = Monster {
                id:          i,
                wait_frames: 2 + i / 4,
                walk_steps:  3 + i as u32 / 3,
                walk_right:  i % 3 == 0,
                position:    (i * i) as i64,
                either:      Either::Wait(Wait::new(0)),
            };
            stack.push(monster);
        }

        Self {
            stack,
        }
    }
}

impl Fiber for Legion {
    type Output = ();
    type Yield<'a> = &'a Vec<Monster>
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Output> {
        let mut fbr = self.stack.pop().unwrap();
        let _ = fbr.run();
        self.stack.insert(0, fbr);
        State::Yield(&self.stack)
    }
}

struct Monster {
    id:          u64,
    wait_frames: u64,
    walk_steps:  u32,
    walk_right:  bool,
    position:    i64,
    either:      Either<Wait, Walk>,
}

impl Fiber for Monster {
    type Output = ();
    type Yield<'a> = &'a i64
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Output> {
        match &mut self.either {
            Either::Wait(wait) => {
                if let State::Done(_) = wait.run() {
                    self.either = Either::Walk(Walk::new(
                        self.walk_steps,
                        self.walk_right,
                        self.position,
                    ));
                }
            }
            Either::Walk(walk) => {
                if let State::Yield(pos) = walk.run() {
                    self.position = *pos;
                } else {
                    self.walk_right = !self.walk_right;
                    self.either = Either::Wait(Wait::new(self.wait_frames))
                }
            }
        }
        State::Yield(&self.position)
    }
}

enum Either<T, K> {
    Wait(T),
    Walk(K),
}

struct Wait {
    wait_frames: u64,
    count:       u64,
}

impl Wait {
    fn new(wait_frames: u64) -> Self {
        Self {
            wait_frames,
            count: 0,
        }
    }
}

impl Fiber for Wait {
    type Output = ();
    type Yield<'a> = ()
    where
        Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Output> {
        if self.count < self.wait_frames {
            self.count += 1;
            State::Yield(())
        } else {
            State::Done(())
        }
    }
}

struct Walk {
    walk_steps: u32,
    walk_right: bool,
    count:      u32,
    pos:        i64,
}

impl Walk {
    fn new(
        walk_steps: u32,
        walk_right: bool,
        init_pos: i64,
    ) -> Self {
        Self {
            walk_steps,
            walk_right,
            count: 0,
            pos: init_pos,
        }
    }
}

impl Fiber for Walk {
    type Output = i64;
    type Yield<'a> = &'a i64 where Self: 'a;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Output> {
        if self.count < self.walk_steps {
            self.count += 1;
            self.pos += if self.walk_right { 1 } else { -1 };
            State::Yield(&self.pos)
        } else {
            State::Done(self.pos)
        }
    }
}
