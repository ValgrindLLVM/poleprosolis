#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

const char* check(void);

int main(void) {
    const char *err;

    if ((err = check()) != 0) {
        fputs(err, stdout);
        fputc('\n', stdout);
        return 1;
    }

    printf("%d %d %d", STDIN_FILENO, F_SETFL, O_NONBLOCK);

    return 0;
}

const char *check(void) {
    int fd, re;

    fd = open("/dev/null", O_RDONLY);
    if (-1 == fd) {
        perror("open(\"/dev/null\")");
        return "failed to open test file";
    }

    if (-1 == fcntl(fd, F_SETFL, O_NONBLOCK)) {
        perror("fcntl(fd, F_SETFL, O_NONBLOCK)");
        return "failed to execute fcntl";
    }

    return 0; /* ok */
}
