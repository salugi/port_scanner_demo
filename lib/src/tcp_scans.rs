use std::io;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;



/*

connect scan

 */
pub fn tcp_connect_scan(address:&str, ports:Vec<i32>)->Result<Vec<i32>, io::Error>{

    let mut open_port_vector:Vec<i32> = vec![];
    let duration = Duration::from_millis(80);



    for port in ports {

        let new_string = format!("{}:{}", address, port);
        let server_details:SocketAddr = new_string.parse().expect("unable to parse");

        if TcpStream::connect_timeout(&server_details,duration).is_ok() {

            open_port_vector.push(port);

        }

    }

    Ok(open_port_vector)

}


/*

connect timeout scan

 */
pub fn tcp_timeout_scan(address:&str, ports:Vec<i32>, timeout:u64)->Result<Vec<i32>, io::Error>{

    let mut open_port_vector:Vec<i32> = vec![];
    let duration = Duration::from_millis(timeout);

    for port in ports {

        let new_string = format!("{}:{}", address, port);
        let server_details:SocketAddr = new_string.parse().expect("unable to parse");

        if TcpStream::connect_timeout(&server_details,duration).is_ok() {

            open_port_vector.push(port);

        }

    }

    Ok(open_port_vector)

}