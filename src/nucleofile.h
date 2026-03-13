#ifndef NUCLEOFILE_H
#define NUCLEOFILE_H

typedef struct {
    char liteRun; /* If system is week, do a lite run */
    int memAvail; /* Max memory available in KB */
} nucfSystemInfo;

void nucf_err(int errno);
int nucf_startup(nucfSystemInfo *sysInfo);
int nucf_run_app(int argc, char **argv, nucfSystemInfo sysInfo);
int nucf_shutdown(nucfSystemInfo *sysInfo);

#endif /* NUCLEOFILE_H */
