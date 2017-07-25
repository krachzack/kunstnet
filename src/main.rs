mod club;

use club::Club;

use std::net::UdpSocket;
use std::str;
use std::env;

enum RunMode {
    Server,
    TestClient(String)
}

fn main() {
    match determine_run_mode() {
        RunMode::TestClient(server_addr) => client(server_addr),
        RunMode::Server => server()
    }
}

fn server() {
    let mut club = Club::bind();

    loop {
        club.process_next().expect("IO error");
    }
}

fn determine_run_mode() -> RunMode {
    // If got any arguments, assume it is the name of the hostname without port
    // of the server to connect to
    let possible_server_addr = env::args().skip(1).next();

    if let Some(server_addr) = possible_server_addr {
        // the server name test should map to the uberspace hosted one
        match &server_addr[..] {
            "test"  => RunMode::TestClient(String::from("185.26.156.19:65318")),
            "local" => RunMode::TestClient(String::from("127.0.0.1:65318")),
            _       => RunMode::TestClient(format!("{}:{}", server_addr, club::PORT))
        }
    } else {
        RunMode::Server
    }
}

fn client(server_addr : String) {
    println!("Client connecting to central node {}…", server_addr);

    let mut buf = [0; 256];

    let msg_srv = "please connect me with somebody";
    let msg = "brau!";
    let client_to_srv = UdpSocket::bind("0.0.0.0:0").expect("could not bind client");
    client_to_srv.send_to(msg_srv.as_bytes(), server_addr).expect("Could not send to server");

    {
        let recv_len = client_to_srv.recv(&mut buf).expect("Got nothing back 1");
        let received_text = str::from_utf8(&buf[0..recv_len]).expect("Got invalid utf-8");

        println!("[1] Client received: {} from central node", received_text);

        client_to_srv.connect(received_text).expect("could not connect to other client");
    }
    
    client_to_srv.send(msg.as_bytes()).expect("Could not send to other client");

    {
        let (recv_len, sender_addr) = client_to_srv.recv_from(&mut buf).expect("Got nothing back 2");
        let received_text = str::from_utf8(&buf[0..recv_len]).expect("Got invalid utf-8");

        println!("[2] Client received: {} from {}", received_text, sender_addr);
    }
}