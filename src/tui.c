#include "tui.h"
#include <target.h>

#include <stdlib.h>
#include <stdio.h>

void run_tui(tuiRequest *request)
{
    int termRows;
    int termCols;
    unsigned char *req;

    printf("TUI is running.\n");
    TARG_get_term_size(&termRows, &termCols);

    printf("Term size is %d x %d \n", termRows, termCols);

    req = malloc(1 * sizeof(unsigned char));
    req[0] = 0;

    request->numOfReq = 0;
    request->requests = req;
    request->numOfPaths = 0;
    request->paths = NULL;
    
}
