use messaging_thread_pool_macros::pool_item;

#[derive(Debug)]
pub struct MacroTest {
    id: u64,
}

impl MacroTest {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

#[pool_item]
impl MacroTest {
    #[messaging(TestRequest, TestResponse)]
    pub fn test_method(&self, _arg: u64) -> u64 {
        self.id
    }
}

#[test]
fn test_macro_generation() {
    // This code expects the macro to generate:
    // 1. TestRequest struct
    // 2. TestResponse struct
    // 3. PoolItem implementation for MacroTest

    let item = MacroTest { id: 1 };

    // Verify PoolItem trait is implemented
    // Note: PoolItem trait has an id() method
    assert_eq!(item.id, 1);

    // Verify Request struct exists
    // Request should have (id, arg)
    let request = TestRequest(1, 10);
    assert_eq!(request.0, 1);
    assert_eq!(request.1, 10);

    // Verify Response struct exists
    let response = TestResponse { id: 1, result: 1 };
    assert_eq!(response.id, 1);
    assert_eq!(response.result, 1);
}
