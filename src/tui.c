#include "tui.h"
#include <target.h>

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/* Rewrite printf with a custom fwrite */
void printxt(char* str, int len)
{
    fwrite(str, 1, len, stdout);
}

void throw_err(int err)
{
    printf(REDBOLD "Error: %d" RESET, err);
}

static int same_word(char* sample, char *key)
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

void run_tui(Toolset tools)
{
    int termRows;
    int termCols;

    // enum Command req[12];
    // unsigned short numReq = 0;
    // char paths[20];
    // unsigned short numPaths = 0;

    int tuiShouldRun = 1;
    char buffer[256];
    char *a;
    int waste;

    printxt("nucleofile V0.1\nAangstrom interpreter running.\n", 47);
    while (tuiShouldRun) {
        TARG_get_term_size(&termRows, &termCols);
        printxt(INT_NEWLN, 11);

        /* Cases to close interpreter */
        a = fgets(buffer, sizeof(buffer), stdin);
        if (a != buffer) {
            tuiShouldRun = 0;
        }

        if (same_word(buffer, "quit\n")) {
            tuiShouldRun = 0;
        }

        if (same_word(buffer, "clear\n")) {
            printf(CLEAR CURSOR_HOME);
        }

        /* Operator and tool calls */
        if (same_word(buffer, "func1\n")) {
            waste = tools.func[COALESCE](1);
            if (waste) {
                throw_err(waste);
            }
        }

        if (same_word(buffer, "func2\n")) {
            waste = tools.func[ANALYSIS](2);
            if (waste) {
                throw_err(waste);
            }
        }
    }
}
