// Closure通常指词法闭包，是一个持有外部环境的函数。外部环境指针闭包定义是所处词法作用域
// 在函数式编程范式中称为自由变量，指并不是在闭包内定义的变量。将自由变量和自身绑定的函数就是闭包

// 返回闭包;
// fn counter(x: i32) -> Box<Fn(i32) -> i32> {
//     Box::new(move |n: i32| n + x)
// }
// Fn : trait
fn counter(x: i32) -> impl Fn(i32) -> i32 {
    Box::new(move |n: i32| n + x)
}

// 闭包特性：1: 延迟执行 1: 补货环境变量
// 基本语法：|a: i32, b: i32| -> i32 { a + b};
// 闭包函数没有参数只有捕获的自由变量时，管道符里的参数也可以省略
// let add = || a + b;