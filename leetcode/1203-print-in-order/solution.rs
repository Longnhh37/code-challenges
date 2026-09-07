struct Gate {
    signaled: Mutex<bool>,
    cond: Condvar,
}

impl Gate {
    fn new() -> Self {
        Self {
            signaled: Mutex::new(false),
            cond: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut guard = self.signaled.lock().unwrap();
        while !*guard {
            guard = self.cond.wait(guard).unwrap();
        }
    }

    fn open(&self) {
        let mut guard = self.signaled.lock().unwrap();
        *guard = true;
        self.cond.notify_all();
    }
}

struct Foo {
    gate_12: Gate,
    gate_23: Gate,
}

impl Foo {
    fn new() -> Self {
        Self {
            gate_12: Gate::new(),
            gate_23: Gate::new(),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        print_first();
        self.gate_12.open();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        self.gate_12.wait();
        // Do not change this line
        print_second();
        self.gate_23.open();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        self.gate_23.wait();
        // Do not change this line
        print_third();
    }
}
