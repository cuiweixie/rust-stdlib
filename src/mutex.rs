use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个整数计数器，用Mutex包装以实现线程安全
    // 用Arc (Atomic Reference Counting) 包装以在线程间共享所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 创建10个线程
    for _ in 0..10 {
        // 克隆Arc，增加引用计数而不是复制数据
        let counter = Arc::clone(&counter);

        // 生成新线程
        let handle = thread::spawn(move || {
            // 在循环中对计数器加1，重复100次
            for _ in 0..100 {
                // 锁定Mutex以获取对数据的独占访问
                let mut num = counter.lock().unwrap();
                *num += 1;
                // 锁在这里自动释放，当num离开作用域时
            }
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终计数器值
    println!("最终计数器值: {}", *counter.lock().unwrap());
    // 预期输出: "最终计数器值: 1000"
}
