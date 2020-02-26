extern crate gprl_lib_demo;

#[test]
fn integral_test_compute() {
    let add = gprl_lib_demo::compute_add(1000,999);
    let sub = gprl_lib_demo::compute_sub(999,1000);
    
    assert_eq!(add + sub, 1998);
}
