use std::sync::OnceLock;
use std::thread;

static DATA: OnceLock<String> = OnceLock::new();

fn get_data() -> &'static String {
    DATA.get_or_init(|| {
        println!("初始化数据 - 这条消息只会打印一次");
        String::from("Hello, world!")
    })
}

fn main() {
    let handles: Vec<_> = (0..10)
        .map(|id| {
            thread::spawn(move || {
                let data = get_data();
                println!("线程 {}: 数据是 \"{}\"", id, data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
