#include "tui.h"
#include <target.h>

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int isMatch(char *sample, char *target)
{
    int i = 0;
    int match = 0;

    while (target[i] != '\n') {
        if (sample[i] == '\n') {
            match = 0;
            break;
        }

        if (sample[i] == target[i]) {
            match = 1;
        }
        i++;
    }

    return match;
}

void run_tui(tuiRequest *request)
{
    int termRows;
    int termCols;

    int i = 0;

    unsigned char req[12];
    unsigned short numReq = 0;
    char paths[20];
    unsigned short numPaths = 0;

    int tuiShouldRun = 1; /* 1 is no error */
    char buffer[120];
    char *a;

    printf("TUI is running...\n");
    while (tuiShouldRun) {
        //buffer[strcspn(buffer, '\n')] = 0;
        TARG_get_term_size(&termRows, &termCols);
        printf("Term size is %d x %d \n", termRows, termCols);

        a = fgets(buffer, sizeof(buffer), stdin);
        if (a != buffer) {
            tuiShouldRun = 0;
        }


        if (buffer[0] == 'q' || buffer[1] == 'q') {
            tuiShouldRun = 0;
        }


        //printf("Made it past fgets");

        // if (!strcmp(buffer, "close\n") || !strcmp(buffer, "quit\n")) {
        //     //tuiShouldClose = 1;
        //     break;
        // }
        //
        // if (isMatch(buffer, "file \n")) {
        //     numReq++;
        //     req[0] = 1;
        //     i = 0;
        //     numPaths = 1;
        //     while (buffer[i + 5] != '\n' && i < 20) {
        //         paths[i] = buffer[i + 5];
        //     }
        // }
    }

    request->numOfReq = numReq;
    request->requests = req;
    request->numOfPaths = numPaths;
    request->paths = paths;
}
