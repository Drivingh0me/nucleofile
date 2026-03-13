#include <target.h>

#include <sys/ioctl.h>
#include <stdio.h>
#include <unistd.h>

void TARG_get_term_size(int *rows, int *cols)
{
    struct winsize window;
    int success;

    int a = ioctl(STDIN_FILENO, TIOCGWINSZ, &window);

    *rows = window.ws_row;
    *cols = window.ws_col;
}

