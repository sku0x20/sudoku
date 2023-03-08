#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::panic::catch_unwind;
use std::sync::Mutex;

static mut setupCalled: bool = false;
static mut testCodeCalled: bool = false;
static mut teardownCalled: bool = false;

static lock: Mutex<i32> = Mutex::new(0);

fn setup() {
    unsafe {
        setupCalled = true;
    }
}

fn teardown() {
    unsafe {
        assert!(testCodeCalled);
        teardownCalled = true
    }
}

#[test]
fn withoutPanic() {
    let _guard = lock.lock().unwrap();

    withFixture(setup, teardown, || {
        unsafe {
            assert!(setupCalled);
            testCodeCalled = true;
        }
    });
    unsafe {
        assert!(teardownCalled);
    }
}

#[test]
fn withPanic() {
    let _guard = lock.lock().unwrap();

    let result = catch_unwind(|| {
        withFixture(setup, teardown, || {
            unsafe {
                assert!(setupCalled);
                testCodeCalled = true;
            }
            panic!("should panic");
        });
    });
    assert!(result.is_err());

    unsafe {
        assert!(teardownCalled);
    }
}

// support for test fixture
// https://stackoverflow.com/a/38254435
struct TestFixture {}

impl TestFixture {}

impl Drop for TestFixture {
    fn drop(&mut self) {
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

fn scopedImpl(setup: fn(), teardown: fn(), testCode: fn()) {
    setup();
    testCode();
    teardown();
}

// is drop called for static also.
// we can try the same thing for class setup and teardown.