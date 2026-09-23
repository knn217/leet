pub struct Foo {
    state: Mutex<usize>,
    cv: Condvar,
}

impl Foo {
    pub fn new() -> Self {
        Foo {
            state: Mutex::new(1),
            cv: Condvar::new(),
        }
    }

    pub fn first(&self, print_first: impl Fn()) {
        print_first();
        *self.state.lock().unwrap() = 2;
        self.cv.notify_all();
    }

    pub fn second(&self, print_second: impl Fn()) {
        let mut step = self.state.lock().unwrap();
        // Automatically loops while the predicate returns true (*step < 2)
        step = self.cv.wait_while(step, |s| *s < 2).unwrap();

        print_second();

        *step = 3;
        self.cv.notify_all();
    }

    pub fn third(&self, print_third: impl Fn()) {
        let mut step = self.state.lock().unwrap();
        step = self.cv.wait_while(step, |s| *s < 3).unwrap();

        print_third();
    }
}