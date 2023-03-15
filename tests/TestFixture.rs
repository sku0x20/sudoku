#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]


// support for test fixture
// https://stackoverflow.com/a/38254435
struct TestFixture {
    setup: fn(),
    teardown: fn(),
    block: fn(),
}

impl TestFixture {
    const fn new(setup: fn(), teardown: fn(), block: fn()) -> TestFixture {
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
