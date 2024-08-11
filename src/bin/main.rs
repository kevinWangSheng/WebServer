use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use WebServer_kevin::ThreadPool;


fn main() {
    // 创建一个TCP连接,默认本地，端口7878
    let connect = TcpListener::bind("127.0.0.1:7878");
    let pool = ThreadPool::new(4);
    for stream in connect.unwrap().incoming().take(3){
        let stream = stream.unwrap();
        println!("new connection: {:?}", stream);
        pool.execute(||{
            handle_connect(stream);
        });
    }
}

pub fn handle_connect(mut stream:TcpStream){

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET /something-else HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/index.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/index.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "src/html/404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("HTTP/1.1 \r\ncontentLenght:{}, 200 OK \r\n\r\n{}",content.len(),content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("{}",String::from_utf8_lossy(&buffer[..]));
}
