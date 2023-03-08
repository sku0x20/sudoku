#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::panic::catch_unwind;
use std::sync::Mutex;

static mut functionsCalled: Vec<String> = Vec::new();
static lock: Mutex<u32> = Mutex::new(0);

fn setup() {
    unsafe {
        functionsCalled = Vec::new();

        functionsCalled.push(String::from("setup"));
    }
}

fn teardown() {
    unsafe {
        functionsCalled.push(String::from("teardown"));
    }
}

#[test]
fn withoutPanic() {
    let _guard = lock.lock().unwrap();

    withFixture(setup, teardown, || {
        unsafe {
            functionsCalled.push(String::from("testCode"));
        }
    });

    unsafe {
        assert_eq!(functionsCalled[0], "setup");
        assert_eq!(functionsCalled[1], "testCode");
        assert_eq!(functionsCalled[2], "teardown");
    }
}

#[test]
fn withPanic() {
    let _guard = lock.lock().unwrap();

    let result = catch_unwind(|| {
        withFixture(setup, teardown, || {
            unsafe {
                functionsCalled.push(String::from("testCode"));
            }
            panic!("should panic");
        });
    });
    assert!(result.is_err());

    unsafe {
        assert_eq!(functionsCalled[0], "setup");
        assert_eq!(functionsCalled[1], "testCode");
        assert_eq!(functionsCalled[2], "teardown");
    }
}

// support for test fixture
// https://stackoverflow.com/a/38254435
struct TestFixture {
    setup: fn(),
    teardown: fn(),
    block: fn(),
}

impl TestFixture {
    fn new(setup: fn(), teardown: fn(), block: fn()) -> TestFixture {
        TestFixture {
            setup,
            teardown,
            block,
        }
    }

    fn run(&self) {
        (self.setup)();
        (self.block)();
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        (self.teardown)();
    }
}

pub fn withFixture(setUp: fn(), tearDown: fn(), testCode: fn()) {
    scopedImpl(setUp, tearDown, testCode)
}

pub fn withTestFixture(setUp: fn(), tearDown: fn(), testCode: fn()) {
    scopedImpl(setUp, tearDown, testCode)
}

pub fn scoped(setup: fn(), teardown: fn(), testCode: fn()) {
    scopedImpl(setup, teardown, testCode)
}

#[inline]
fn scopedImpl(setup: fn(), teardown: fn(), testCode: fn()) {
    let testFixture = TestFixture::new(setup, teardown, testCode);
    testFixture.run();
}

// is drop called for static also.
// we can try the same thing for class setup and teardown.