use wasi::{fd_write, Ciovec};

fn main() {
    let data = [Ciovec {
        buf: "Hello, World!\n".as_ptr(),
        buf_len: 14,
    }];
    unsafe {
        fd_write(1, &data).unwrap();
    }
}
