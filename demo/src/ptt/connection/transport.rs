#[async_trait]
pub trait Transport {
    async fn send(&mut self, data: Vec<u8>) -> Result<()>;
    async fn recv(&mut self) -> Result<Vec<u8>>;
}
