#include "nucleofile.h"

#include "math.h"
#include "outFiles.h"
#include "tui.h"

#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int length;
    int lines;
    double *data;
} nucfData;

typedef struct {
    int sets;
    nucfData *data_arr;
} nucfDataset;

static void cleanup()
{
    printf("Cleaning up.\n");
}

int function1(int a)
{
    printf("Function%d ran.\n", a);
    return 0;
}

int function2(int a)
{
    printf("Function%d ran.\n", a);
    return 0;
}

void nucf_err(int errno)
{
    if (errno < 0) {
        /* Uses ANSI escape sequence to color error message. */
        printf(REDBOLD "FATAL ERROR: %d" RESET "\n", errno);
        cleanup();
        exit(errno);
    } else {
        printf(REDBOLD "RECOVERABLE ERROR: %d BECAME FATAL.\n" RESET
               "This is likely a bug.\n", errno);
        cleanup();
        exit(errno);
    }
}

static int parse_args(int argc, char **argv, int *runTui)
{
    *runTui = 0;
    if (argc == 1) {
        *runTui = 1;
    }

    if (argc > 1) {
        if (argv[2][0] == '-') {
            if (argv[2][1] == 't') {
                *runTui = 1;
            }
        }
    }

    return 0;
}

static void nucf_export()
{
    printf("Exporting.\n");
}

static int nucf_run_analysis(nucfDataset data)
{
    char should_export = 0;
    int status;

    if (should_export) {
        nucf_export();
    }

    status = fit();
    if (status) {
        return 3;
    }

    return 0;
}

static int nucf_coalesce_data(nucfDataset *dataset)
{
    printf("Coalescing data.\n");

    return 0;
}

int nucf_startup(nucfSystemInfo *sysInfo)
{
    // Check system info.
    return 0;
}

int nucf_run_app(int argc, char **argv, nucfSystemInfo sysInfo)
{
    int status = -1;
    int runTui;
    nucfDataset dataset;
    Toolset tools;

    tools.func[COALESCE] = function1;
    tools.func[ANALYSIS] = function2;

    status = parse_args(argc, argv, &runTui);
    if (status) {
        return status;
    }

    if (runTui) {
        run_tui(tools);
    }

    // status = nucf_coalesce_data(userCommand, &dataset);
    // if (status) {
    //     return status;
    // }
    //
    // status = nucf_run_analysis(dataset);
    // if (status) {
    //     return status;
    // }

    return 0;
}

int nucf_shutdown(nucfSystemInfo *sysInfo)
{
    cleanup();

    return 0;
}
