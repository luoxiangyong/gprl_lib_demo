//! 本库是一些演示简单计算的函数集合。
//!
//! # 示例
//! ```
//! assert_eq!(gprl_lib_demo::compute_add(2,2), 4);
//! ```


#![feature(test)]

extern crate test;

use test::Bencher;


///　计算两个函数的和
///
///　# 示例
///
///```
///　assert_eq!(gprl_lib_demo::compute_add(1,1), 2);
///```
/// # 参数
///   * `a` 第一个整数
///   * `b` 第二个整数
#[allow(dead_code)]
pub fn compute_add(a:i32,b:i32) -> i32 {
    let s = a + b;
    println!("用Rust计算add的结果是:{}",s);
    s
}


///　计算两个函数的差
///
///　# 示例
///
///```
///　assert_eq!(gprl_lib_demo::compute_sub(1,1), 0);
///```
/// # 参数
///   * `a` 第一个整数
///   * `b` 第二个整数
#[allow(dead_code)]
pub fn compute_sub(a:i32,b:i32) -> i32 {
    let s = a - b;
    println!("用Rust计算sub的结果是:{}",s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_add() {
        assert_eq!(compute_add(1000,999), 1999);
    }
    
    #[test]
    fn test_compute_sub() {
        assert_eq!(compute_sub(999,1000), -1);
    }
    
    #[bench]
    fn bench_compute_add(b:&mut Bencher) {
        b.iter(|| compute_add(1000,999));
    }
    
    #[bench]
    fn bench_compute_sub(b:&mut Bencher) {
        b.iter(|| compute_sub(1000,999));
    }
}
