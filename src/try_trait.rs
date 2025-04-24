#![feature(try_trait_v2)]

use std::ops::{ControlFlow, FromResidual, Try};
use std::convert::Infallible;

// 定义一个简单的自定义结果类型
#[derive(Debug)]
enum MyResult<T> {
    Success(T),
    Error(String),
}

// 实现 FromResidual 以支持 ? 运算符
impl<T> FromResidual<Result<Infallible, String>> for MyResult<T> {
    fn from_residual(residual: Result<Infallible, String>) -> Self {
        match residual {
            Err(e) => MyResult::Error(e),
            _ => unreachable!(),
        }
    }
}

// 实现 Try trait
impl<T> Try for MyResult<T> {
    type Output = T;
    type Residual = Result<Infallible, String>;

    fn from_output(output: Self::Output) -> Self {
        MyResult::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            MyResult::Success(v) => ControlFlow::Continue(v),
            MyResult::Error(e) => ControlFlow::Break(Err(e)),
        }
    }
}

// 使用我们的自定义类型和 ? 运算符
fn might_fail(value: i32) -> MyResult<i32> {
    if value < 0 {
        return MyResult::Error("Value must be non-negative".to_string());
    }
    MyResult::Success(value)
}

fn process(value: i32) -> MyResult<i32> {
    // 使用 ? 运算符处理 might_fail 的结果
    let result = might_fail(value)?;

    // 如果 might_fail 返回 Error，process 会提前返回该 Error
    // 否则，我们可以使用 result 变量

    MyResult::Success(result * 2)
}

fn main() {
    let positive = process(10);
    let negative = process(-5);

    println!("Positive result: {:?}", positive);
    println!("Negative result: {:?}", negative);
}
