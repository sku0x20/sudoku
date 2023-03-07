#![allow(non_snake_case)]

// support for test fixture
// https://stackoverflow.com/a/38254435
struct TestFixture {}

impl TestFixture {}

impl Drop for TestFixture {
    fn drop(&mut self) {}
}

pub fn withFixture(setUp: fn(), tearDown: fn(), testCode: fn()) {
    withTestFixture(setUp, tearDown, testCode)
}

fn withTestFixture(setup: fn(), teardown: fn(), testCode: fn()) {
    setup();
    testCode();
    teardown();
}

static mut setupCalled: bool = false;
static mut testCodeCalled: bool = false;
static mut teardownCalled: bool = false;

fn setup() {
    unsafe {
        setupCalled = true;
    }
}

// idk, how to test if teardown is called.
fn teardown() {
    unsafe {
        assert!(testCodeCalled);
        teardownCalled = true
    }
}

#[test]
fn withoutPanic() {
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

// Would be better if it's like TestFixture::scoped,
// or withFixture is also fine.
// export both.