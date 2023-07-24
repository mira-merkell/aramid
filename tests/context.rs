use std::{
    sync::{
        mpsc::{
            sync_channel,
            Receiver,
            SyncSender,
        },
        Arc,
        Mutex,
        MutexGuard,
    },
    thread::{
        self,
        JoinHandle,
    },
};

struct MockFiber {
    data: u8,
}

impl MockFiber {
    fn run(
        &mut self,
        cx: &mut Context,
    ) -> u8 {
        for i in 0..3 {
            println!("Hello from dual space");
            cx.r#yield(i);
        }
        self.data
    }
}

struct Context {
    tx: SyncSender<u8>,
    rx: Receiver<()>,
}

impl Context {
    fn r#yield(
        &mut self,
        val: u8,
    ) {
        self.tx.send(val).unwrap();
        let _ = self.rx.recv().unwrap();
    }
}

struct Executor {
    handle: JoinHandle<u8>,
    tx:     SyncSender<()>,
    rx:     Receiver<u8>,
}

impl Executor {
    fn wrap(mut fbr: MockFiber) -> Self {
        let (tx_yld, rx_yld) = sync_channel(1);
        let (tx_ctl, rx_ctl) = sync_channel(1);
        let mut cx = Context {
            tx: tx_yld,
            rx: rx_ctl,
        };
        let handle = thread::spawn(move || {
            let _ = cx.rx.recv().unwrap();
            fbr.run(&mut cx)
        });
        Self {
            handle,
            tx: tx_ctl,
            rx: rx_yld,
        }
    }

    fn run(&mut self) -> u8 {
        self.tx.send(()).unwrap();
        self.rx.recv().unwrap()
    }
}

#[test]
fn mockup_fiber_01() {
    let fbr = MockFiber {
        data: 10
    };
    let mut exec = Executor::wrap(fbr);

    println!("Hello from primary space");
    assert_eq!(exec.run(), 0);
    println!("Hello from primary space");
    assert_eq!(exec.run(), 1);
    assert_eq!(exec.run(), 2);
}
