use integ_test_example;

#[test]
fn files_test1() {
    assert_ne!(integ_test_example::get_process_id(),0,"Error in code");
}

#[test]
fn files_test2() {
    assert_eq!(1+1, 2);
}

#[test]
// 假如说这个函数是计算密集型，每次运行都要花费很长时间，那么可以使用ignore来忽略这个测试
// 之后可以使用 cargo test -- --ignored 来运行被忽略的测试
#[ignore]
fn process_test1() {
    assert!(true);
}