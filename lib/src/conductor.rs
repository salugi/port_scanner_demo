use crate::tcp_scans::{tcp_connect_scan, tcp_timeout_scan};
use crate::records::TcpScanRecord;

use std::{io};
use std::net::{IpAddr};
use std::str::{FromStr};
use dns_lookup::{lookup_host};

pub fn conduct_connect_scan(addr:&str, ports:Vec<i32>) ->Result<TcpScanRecord,io::Error> {

    let sanitized_host = host_sanitizer(addr);
    let open_ports = tcp_connect_scan(sanitized_host.clone(), ports)?;
    let record = TcpScanRecord {
        host : sanitized_host,
        open_ports
    };

    Ok(record)

}

pub fn conduct_timeout_scan(addr:&str, ports:Vec<i32>, timeout:u64) ->Result<TcpScanRecord,io::Error> {

    let sanitized_host = host_sanitizer(addr);
    let open_ports = tcp_timeout_scan(&sanitized_host, ports,timeout)?;
    let record = TcpScanRecord {
        host : sanitized_host,
        open_ports
    };

    Ok(record)

}

pub fn conduct_host_lookup(addr:&str)->Result<Vec<String>, io::Error>{

    let ip_vecs : Vec<std::net::IpAddr> = lookup_host(addr).unwrap();
    let mut return_array: Vec<String> = vec![];

    for ip in ip_vecs {
        return_array.push(ip.to_string())
    }


    Ok(return_array)

}

fn host_sanitizer(addr:&str)->String{

    match IpAddr::from_str(addr) {

        Err(_) => {

            let ip_vecs : Vec<std::net::IpAddr> = lookup_host(addr).unwrap();

            ip_vecs[0].to_string()

        },

        Ok(IpAddr::V4(..)) => addr.to_string(),
        Ok(IpAddr::V6(..)) => addr.to_string(),

     }

}



