#include <sys/uio.h>

int main() {
    struct iovec iov[1];
    iov[0].iov_base = "Hello, World!\n";
    iov[0].iov_len = 14;

    writev(1, iov, 1);
    return 0;
}
