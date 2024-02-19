use std::{io::{self, Read, Result, Write}, net::TcpStream};

use ffi::Event;
use poll::Poll;

mod ffi;
mod poll;

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams = Vec::new();
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url = format!("/{delay}/request-{i}");
        let request = get_req(&url);
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;
        poll.registry().register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("Timeout or unexpected event notification");
            continue;
        }

        handled_events += handle_events(&events, &mut streams)?;
    }

    println!("Done");
    Ok(())
}

fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled = 0;
    for event in events {
        let idx = event.token();
        let mut data = vec![0u8; 0x1000];

        loop {
            match streams[idx].read(&mut data) {
                Ok(n) if n == 0 => {
                    handled += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECV: {event:?}");
                    println!("{txt}\n----------\n");
                }

                Err(e) 
                    if e.kind() == io::ErrorKind::WouldBlock 
                    || e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled)
}

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n
        \r\n"
    )
}