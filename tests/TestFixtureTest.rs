#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

/// --- tests for TestFixture ---

use std::panic::catch_unwind;
use std::sync::Mutex;
use crate::TestFixture::withFixture;

mod TestFixture;

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