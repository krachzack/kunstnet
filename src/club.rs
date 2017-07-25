
use std::io;
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::str;

pub const PORT : u16 = 65318;

#[derive(Debug)]
pub struct Club {
    pending: Option<SocketAddr>,
    socket: UdpSocket,
}

impl Club {

    pub fn bind() -> Club {
        println!("Creating server");

        let bind_addr = format!("0.0.0.0:{}", PORT);
        
        Club {
            pending: None,
            socket: UdpSocket::bind(bind_addr).expect("could not bind server"),
        }
    }

    pub fn process_next(&mut self) -> io::Result<()> {
        println!("Server receiving");
        let mut buf = [0; 256];
        let (recv_len, sender_addr) = self.socket.recv_from(&mut buf)?;

        let received_text = str::from_utf8(&buf[0..recv_len]).expect("Got invalid utf-8");

        println!("Received:\n{}\nFrom: {}", received_text, sender_addr);

        self.pending = match self.pending.take() {
            // If already has pending and this pending has different address,
            // send each recipient the address of the other and set pending to None again
            Some(pending_addr) if pending_addr != sender_addr => {
                let pending_addr_serialized = format!("{}", pending_addr);
                let pending_addr_serialized = pending_addr_serialized.as_bytes();
                let sender_addr_serialized = format!("{}", sender_addr);
                let sender_addr_serialized = sender_addr_serialized.as_bytes();
                
                // Send both clients the address of the other peer
                self.socket.send_to(pending_addr_serialized, sender_addr).unwrap();
                self.socket.send_to(sender_addr_serialized, pending_addr).unwrap();

                None
            },
            // If no one is pending or the sender was the same as the pending sender
            // set the sender as pending
            _ => Some(sender_addr)
        };

        Ok(())
    }
}
