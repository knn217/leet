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
        // printFirst() outputs "first". Do not change or remove this line.
        print_first();
        *self.state.lock().unwrap() = 2;
        self.cv.notify_all();
    }

    pub fn second(&self, print_second: impl Fn()) {
        let mut step = self.state.lock().unwrap();
        // Automatically loops while the predicate returns true (*step < 2)
        step = self.cv.wait_while(step, |s| { return*s < 2; }).unwrap();

        // printSecond() outputs "second". Do not change or remove this line.
        print_second();

        *step = 3;
        self.cv.notify_all();
    }

    pub fn third(&self, print_third: impl Fn()) {
        let mut step = self.state.lock().unwrap();
        step = self.cv.wait_while(step, |s| { return*s < 3; }).unwrap();

        // printThird() outputs "third". Do not change or remove this line.
        print_third();
    }
}