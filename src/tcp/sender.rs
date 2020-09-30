use std::net::UdpSocket;

#[derive(Debug)]
pub struct Sender {
    filepath: String,
    socket: UdpSocket,
}

impl Sender {
    pub fn new(filepath: String, ipv4Addr: &String) -> Sender {
        Sender {
            filepath,
            socket: UdpSocket::bind(ipv4Addr)
                .expect(format!("Couldn't bind sender to {}!", ipv4Addr)),
        }
    }

    fn establish_connection(&self, ipv4Addr: &String) -> std::io::Result<()> {
        {
            let mut socket = UdpSocket::bind(ipv4Addr)?;
            let mut buf = [0; 1000];
            let (amt, src) = socket.recv_from(&mut buf)?;
            let buf = &mut buf[..amt];
            buf.reverse();
            socket.send_to(buf, &src);
        }
        Ok(())
    }
}
