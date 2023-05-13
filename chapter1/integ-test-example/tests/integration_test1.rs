use integ_test_example;

#[test]
fn test1() {
    assert_ne!(integ_test_example::get_process_id(), 0, "Error in code");
}