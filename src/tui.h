#ifndef TUI_H
#define TUI_H

typedef struct {
    unsigned short numOfReq;
    unsigned char *requests;
    unsigned short numOfPaths;
    char **paths;
} tuiRequest;

#define RESET "\033[0m"
#define CLEAR "\033[2J"

/* Foreground colors */
#define BLACK "\033[30m"
#define RED "\033[31m"
#define GREEN "\033[32m"
#define YELLOW "\033[33m"
#define BLUE "\033[34m"
#define MAGENTA "\033[35m"
#define CYAN "\033[36m"
#define WHITE "\033[37m"
#define COLOR256 "\033[38;5;%dm"
#define REDBOLD "\033[1;31m"

void run_tui(tuiRequest *request);

#endif /* TUI_H */
