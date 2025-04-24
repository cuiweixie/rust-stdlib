use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个共享的数据结构，使用RwLock包装
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4]));
    let mut read_handles = vec![];
    let mut write_handles = vec![];

    // 创建5个读取线程
    for i in 0..5 {
        let data = Arc::clone(&data);
        let read_handle = thread::spawn(move || {
            // 循环读取数据3次
            for j in 0..3 {
                // 获取读锁
                let values = data.read().unwrap();
                println!("读取线程 #{} (迭代 #{}): 当前数据 = {:?}", i, j, *values);

                // 模拟一些处理时间
                thread::sleep(Duration::from_millis(50));
                // 读锁在这里自动释放
            }
        });
        read_handles.push(read_handle);
    }

    // 创建2个写入线程
    for i in 0..2 {
        let data = Arc::clone(&data);
        let write_handle = thread::spawn(move || {
            // 循环修改数据2次
            for j in 0..2 {
                // 获取写锁
                let mut values = data.write().unwrap();
                let new_value = values.len() as i32 + 1;
                values.push(new_value);
                println!("写入线程 #{} (迭代 #{}): 添加元素后 = {:?}", i, j, *values);

                // 模拟写入操作的时间
                thread::sleep(Duration::from_millis(100));
                // 写锁在这里自动释放
            }
        });
        write_handles.push(write_handle);
    }

    // 等待所有读取线程完成
    for handle in read_handles {
        handle.join().unwrap();
    }

    // 等待所有写入线程完成
    for handle in write_handles {
        handle.join().unwrap();
    }

    // 最终检查数据
    let final_data = data.read().unwrap();
    println!("最终数据: {:?}", *final_data);
}
