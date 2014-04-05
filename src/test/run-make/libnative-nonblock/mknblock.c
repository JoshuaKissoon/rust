#include <fcntl.h>
#include <stdio.h>

int main() {
    if (fcntl(1, F_SETFL, O_NONBLOCK) == -1) {
        perror("fcntl");
        return 1;
    }
    return 0;
}
