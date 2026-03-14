#include <target.h>

#include <sys/ioctl.h>
#include <stdio.h>
#include <unistd.h>

void TARG_get_term_size(int *rows, int *cols)
{
    struct winsize window;
    int fail;

    fail = ioctl(STDIN_FILENO, TIOCGWINSZ, &window);
    if (fail) {
        *rows = *cols = -1;
        return;
    }

    *rows = window.ws_row;
    *cols = window.ws_col;
}

