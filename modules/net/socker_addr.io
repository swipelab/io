module net;

pub type SocketAddr : struct{
  addr: [u8, 4],
  port: u32,
}