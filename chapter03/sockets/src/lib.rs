#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdout::get_stdout;
use bindings::wasi::sockets::instance_network::instance_network;
use bindings::wasi::sockets::network::{IpAddressFamily, IpSocketAddress, Ipv4SocketAddress};
use bindings::wasi::sockets::tcp::ShutdownType;
use bindings::wasi::sockets::tcp_create_socket::create_tcp_socket;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let network = instance_network();
        let address = IpSocketAddress::Ipv4(Ipv4SocketAddress {
            port: 8080,
            address: (127, 0, 0, 1),
        });
        let stdout = get_stdout();

        let socket = create_tcp_socket(IpAddressFamily::Ipv4).unwrap();

        // await bind
        stdout.blocking_write_and_flush(b"start_bind...").unwrap();
        socket.start_bind(&network, address).unwrap();
        socket.subscribe().block();
        socket.finish_bind().unwrap();

        stdout.blocking_write_and_flush(b"OK!\n").unwrap();

        // await listen
        stdout.blocking_write_and_flush(b"start_listen...").unwrap();
        socket.start_listen().unwrap();
        socket.subscribe().block();
        socket.finish_listen().unwrap();

        stdout.blocking_write_and_flush(b"OK!\n").unwrap();

        // await accept
        stdout.blocking_write_and_flush(b"accepting...").unwrap();
        socket.subscribe().block();
        let (socket, _, output) = socket.accept().unwrap();

        stdout.blocking_write_and_flush(b"Accepted!\n").unwrap();
        output.blocking_write_and_flush(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 21\n\nHello, wasi:sockets!\n").unwrap();
        drop(output);

        socket.shutdown(ShutdownType::Both).unwrap();
        stdout
            .blocking_write_and_flush(b"socket shutdown\n")
            .unwrap();
        drop(socket);
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
