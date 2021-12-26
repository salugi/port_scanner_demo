use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;




/*

connect scan

 */
pub fn tcp_connect_scan(address:&str, ports:Vec<i32>)->Result<Vec<i32>, io::Error>{

    let mut open_port_vector:Vec<i32> = vec![];


    for port in ports {

        let addr_string = format!("{}:{}", address, port);
        let server_details:SocketAddr =  addr_string.parse().expect("unable to parse");

        if TcpStream::connect(&server_details).is_ok() {

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

        let addr_string = format!("{}:{}", address, port);
        let server_details:SocketAddr = addr_string.parse().expect("unable to parse");

        if TcpStream::connect_timeout(&server_details,duration).is_ok() {

            open_port_vector.push(port);

        }

    }

    Ok(open_port_vector)

}

/*

tcp send string

 */
pub fn tcp_send_string(address:String, port:i32, msg:&str, timeout:u64) ->Result<String, io::Error>{


    let addr_string = format!("{}:{}", address, port);
    let server_details:SocketAddr = addr_string.parse().expect("unable to parse");
    let duration = Duration::from_millis(timeout);
    let mut stream =  TcpStream::connect_timeout(&server_details,duration).expect("could not connect");
    let mut response_shell = String::new();
    let req_bytes =     msg.as_bytes();

    stream
        .write_all(req_bytes)
        .expect("couldnt write bytes");

    stream
        .read_to_string(&mut response_shell)
        .expect("could not read string");

Ok(response_shell)
}