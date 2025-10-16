use crate::protos::login::{Login, LoginAck};
use crate::protos::ping::Pong;
use bytes::{BufMut, BytesMut};
use protobuf::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    TcpStream,
    tcp::{OwnedReadHalf, OwnedWriteHalf},
}; // rust-protobuf

#[derive(Debug)]
pub struct TcpClient {
    writer: OwnedWriteHalf,
}

impl TcpClient {
    /// 连接服务器
    pub async fn connect(addr: &str) -> tokio::io::Result<(Self, OwnedReadHalf)> {
        let stream = TcpStream::connect(addr).await?;
        println!("✅ Connected to {}", addr);
        let (reader, writer) = stream.into_split();
        Ok((Self { writer }, reader))
    }

    /// 发送 protobuf 消息
    pub async fn send_proto<T: Message>(&mut self, cmd: u8, proto: &T) -> tokio::io::Result<()> {
        let packet = build_tcp_packet(cmd, proto);
        self.writer.write_all(&packet).await?;
        Ok(())
    }
}

/// 构造 TCP 数据包（rust-protobuf 版本）
pub fn build_tcp_packet<T: Message>(command: u8, protobuf_msg: &T) -> Vec<u8> {
    let body = protobuf_msg.write_to_bytes().unwrap(); // rust-protobuf 序列化

    // 数据区长度 = 命令 + protobuf + CRC
    let data_len = 1 + body.len() + 4;
    let total_len = (data_len as u32).to_be_bytes();
    // let total_len = (data_len as u32).to_le_bytes();

    let mut packet = BytesMut::with_capacity(4 + data_len);
    packet.extend_from_slice(&total_len);
    packet.put_u8(command);
    packet.extend_from_slice(&body);

    // let mut hasher = Hasher::new();
    // hasher.update(&packet);
    // let crc = hasher.finalize().to_be_bytes();
    // // let crc = hasher.finalize().to_le_bytes();
    // packet.extend_from_slice(&crc);

    // 新：
    let crc_val = zte_crc(&packet);
    packet.extend_from_slice(&crc_val.to_be_bytes()); // 默认 big-endian

    println!("最终 TCP 包长度 = {}", packet.len());
    println!("十六进制输出: {}", hex::encode(&packet));
    packet.to_vec()
}
fn zte_crc(data: &[u8]) -> u32 {
    let seed: u32 = 2014;
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for &byte in data {
        a = (a + byte as u32) ^ seed;
        b = (b + a) ^ seed;
    }

    (b << 16) | a
}

/// 解析 TCP 包
pub fn parse_tcp_packet(buffer: &mut BytesMut) -> Option<(u8, Vec<u8>)> {
    const HEADER_LEN: usize = 4;
    const CRC_LEN: usize = 4;
    
    if buffer.len() < HEADER_LEN {
        return None;
    }

    let total_len = u32::from_be_bytes(buffer[..4].try_into().unwrap()) as usize;
    if buffer.len() < HEADER_LEN + total_len {
        return None;
    }

    let packet = buffer.split_to(HEADER_LEN + total_len);
    let payload_without_crc = &packet[..packet.len() - CRC_LEN];
    let recv_crc = u32::from_be_bytes(packet[packet.len() - CRC_LEN..].try_into().unwrap());

    // let mut hasher = Hasher::new();
    // hasher.update(payload_without_crc);
    // let calc_crc = hasher.finalize();
    let calc_crc = zte_crc(payload_without_crc);

    if calc_crc != recv_crc {
        eprintln!(
            "❌ CRC mismatch: calc={:08X}, recv={:08X}",
            calc_crc, recv_crc
        );
        return None;
    }

    let cmd = packet[4];
    let protobuf_bytes = packet[5..packet.len() - CRC_LEN].to_vec();
    Some((cmd, protobuf_bytes))
}

/// 异步接收循环
pub async fn receive_loop(mut reader: OwnedReadHalf) -> tokio::io::Result<()> {
    let mut buffer = BytesMut::with_capacity(4096);
    let mut temp = [0u8; 1024];
    println!("⚠️ receive_loop 1");
    loop {
        println!("⚠️ receive_loop 2");
        let n = reader.read(&mut temp).await?;
        if n == 0 {
            println!("⚠️ 连接关闭");
            break;
        }

        buffer.extend_from_slice(&temp[..n]);

        println!("⚠️ receive_loop 3");

        while let Some((cmd, pb_bytes)) = parse_tcp_packet(&mut buffer) {
            println!(
                "📩 Received cmd=0x{:02X}, size={} bytes",
                cmd,
                pb_bytes.len()
            );
            // 解析 protobuf：
            let msg: LoginAck = LoginAck::parse_from_bytes(&pb_bytes).unwrap();
            println!("LoginAck.result = {:?}", msg.result);
            println!("LoginAck = {:?}", msg);
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn login() -> std::io::Result<()> {
    // 1️⃣ 连接服务器
    let (mut client, reader) = TcpClient::connect("210.51.10.247:2074").await?;

    // 2️⃣ 启动接收任务（后台运行）
    tokio::spawn(async move {
        if let Err(e) = receive_loop(reader).await {
            eprintln!("Receive loop error: {}", e);
        }
    });

    // 3️⃣ 构造 Login protobuf 消息
    let mut login = Login::new();
    login.account = Some("020250802".to_string());
    login.password = Some("020250802".to_string());
    login.platform = Some("Android".to_string());
    login.device = Some("MIJI".to_string());

    // 4️⃣ 构造 TCP 包并发送
    client.send_proto(0x01, &login).await?; // 直接发送 protobuf 消息即可
    // client.send_proto(0x41, &pong).await?; // 直接发送 protobuf 消息即可

    // 5️⃣ 模拟等待一段时间
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(())
}
