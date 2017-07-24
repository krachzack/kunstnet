mod club;

use club::Club;

use std::net::UdpSocket;
use std::str;
use std::thread;

const CLIENT_ADDR : &'static str = "0.0.0.0:0";
const SRV_PORT : u16 = 65318;

fn main() {
    /*let server_handle = thread::spawn(server);
    let client_handle = thread::spawn(client);
    let client_handle2 = thread::spawn(client);

    server_handle.join().expect("Could not join server");
    client_handle.join().expect("Could not join client");
    client_handle2.join().expect("Could not join client 2");*/
    server();
}

fn server() {
    let mut club = Club::bind(SRV_PORT);

    for _ in 0..2 {
        club.process_next().expect("IO error");
    }
}

fn client() {
    let mut buf = [0; 256];

    let msg_srv = "please connect me with somebody";
    let msg = "brau!";
    let client_to_srv = UdpSocket::bind(CLIENT_ADDR).expect("could not bind client");
    client_to_srv.send_to(msg_srv.as_bytes(), format!("0.0.0.0:{}", SRV_PORT)).expect("Could not send to server");

    {
        let (recv_len, sender_addr) = client_to_srv.recv_from(&mut buf).expect("Got nothing back 1");
        let received_text = str::from_utf8(&buf[0..recv_len]).expect("Got invalid utf-8");

        println!("[1] Client received: {} from {}", received_text, sender_addr);

        client_to_srv.connect(received_text).expect("could not connect to other client");
    }
    
    client_to_srv.send(msg.as_bytes()).expect("Could not send to other client");

    {
        let (recv_len, sender_addr) = client_to_srv.recv_from(&mut buf).expect("Got nothing back 2");
        let received_text = str::from_utf8(&buf[0..recv_len]).expect("Got invalid utf-8");

        println!("[2] Client received: {} from {}", received_text, sender_addr);
    }
}