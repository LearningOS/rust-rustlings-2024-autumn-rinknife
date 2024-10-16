// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


// 引入必要的库
use std::sync::mpsc; // 用于多线程中的消息传递
use std::sync::Arc; // 用于跨线程共享所有权
use std::thread; // 用于创建和管理线程
use std::time::Duration; // 用于线程休眠

// 定义一个队列结构体
struct Queue {
    length: u32, // 队列的长度
    first_half: Vec<u32>, // 队列的前半部分
    second_half: Vec<u32>, // 队列的后半部分
}

// 为Queue实现方法
impl Queue {
    // 创建一个新的Queue实例
    fn new() -> Self {
        Queue {
            length: 10, // 初始化长度为10
            first_half: vec![1, 2, 3, 4, 5], // 初始化前半部分
            second_half: vec![6, 7, 8, 9, 10], // 初始化后半部分
        }
    }
}

// 定义一个发送消息的函数
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = tx.clone(); // Clone the transmitter for the first thread
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap(); // Use the cloned transmitter
            thread::sleep(Duration::from_secs(1));
        }
    });

    // No need to clone here, as we can use the original transmitter
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap(); // Use the original transmitter
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // 创建一个新的消息通道
    let (tx, rx) = mpsc::channel();
    // 创建一个新的Queue实例
    let queue = Queue::new();
    // 获取队列的长度
    let queue_length = queue.length;

    // 调用send_tx函数，开始发送队列中的元素
    send_tx(queue, tx);

    // 初始化接收到的消息总数
    let mut total_received: u32 = 0;
    // 循环接收消息，直到通道关闭
    for received in rx {
        println!("Got: {}", received); // 打印接收到的消息
        total_received += 1; // 更新接收到的消息总数
    }

    // 打印接收到的消息总数
    println!("total numbers received: {}", total_received);
    // 断言接收到的消息总数是否等于队列的长度
    assert_eq!(total_received, queue_length);
}
