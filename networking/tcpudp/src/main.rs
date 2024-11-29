mod tcp_udp;
mod tcp_client;

/*
    testing 
    Test-NetConnection -ComputerName localhost -Port 8888
    Test-NetConnection -ComputerName localhost -Port 8888
 */

fn main() {
   
    // tcp_udp::_main();


     tcp_client::client();
}


