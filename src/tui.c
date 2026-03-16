#include "tui.h"
#include <target.h>

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int same_word(char* sample, char *key)
{
    int i = 0;
    while (key[i] != '\n') {
        if (sample[i] != key[i] || sample[i] == '\n') {
            return 0;
        }
        i++;
    }
    if (sample[i] != '\n') {
        return 0;
    }
    return 1;
}

void run_tui(tuiRequest *request)
{
    int termRows;
    int termCols;

    int i = 0;

    enum Command req[12];
    unsigned short numReq = 0;
    char paths[20];
    unsigned short numPaths = 0;

    int tuiShouldRun = 1; /* 1 is no error */
    char buffer[120];
    char *a;

    printf("nucleofile V0.1\n");
    while (tuiShouldRun) {
        TARG_get_term_size(&termRows, &termCols);
        printf(BLUE "> " RESET);

        a = fgets(buffer, sizeof(buffer), stdin);
        if (a != buffer) {
            tuiShouldRun = 0;
        }

        if (same_word(buffer, "quit\n")) {
            tuiShouldRun = 0;
        }
    }

    request->numOfReq = numReq;
    request->requests = req;
    request->numOfPaths = numPaths;
    request->paths = paths;
}
