#[derive(Debug)]
pub struct TcpScanRecord {
    pub host : String,
    pub open_ports : Vec<i32>
}
