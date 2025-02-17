use std::{io::Write, thread, time::Duration};

use nanomsg::{Protocol, Socket};

fn main() {
    let mut socket = Socket::new(Protocol::Pub).unwrap();
    let mut endpoint = socket.bind("ipc:///tmp/name_name_name_name.ipc").unwrap();
    let mut count = 1u32;
    let topic = b"";

    println!("Server is ready.");

    let mut msg = Vec::with_capacity(topic.len() + 16);
    loop {
        let postfix = format!(" #{}", count);
        msg.clear();
        msg.extend_from_slice(topic);
        msg.extend_from_slice(postfix.as_bytes());
        match socket.write_all(&msg) {
            Ok(..) => println!("Published '{:?}'.", msg),
            Err(err) => {
                println!("Server failed to publish '{}'.", err);
                break;
            }
        }
        thread::sleep(Duration::from_millis(400));
        count += 1;
    }

    endpoint.shutdown().unwrap();
}
