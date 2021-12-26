
use lib::conductor::{conduct_timeout_scan, conduct_connect_scan, conduct_host_lookup};

fn main() {

    let hosts = conduct_host_lookup("google.com");
    println!("{:?}", hosts);

    let record = conduct_connect_scan("google.com",vec![80]);
    println!("{:?}", record);

    let record = conduct_timeout_scan("google.com",vec![80], 80);
    println!("{:?}", record);

}
