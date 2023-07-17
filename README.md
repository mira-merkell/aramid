# aramid

Synthetic fibers

- WIP
- Fibers are little state machines that behave like coroutines: when `.run()`,
  they yield first, and then they return. In the meantime, they carry their full
  stack around with them.
- Fibers are a model of concurrent computation. They are static, lightweight and
  particularly well-suited for cooperative multitasking.
