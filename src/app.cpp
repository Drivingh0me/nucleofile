#include <iostream>
#include <string>
#include <vector>
#include <cstring>

#include "app.hpp"
#include "gui.hpp"
#include "platform_specific.h"

enum class Flag {
    Test,
    Silent,
    In // Look for data from std in.
};

struct FlagSet {
    Flag flag;
    std::vector<std::string> params;
};

std::vector<FlagSet> set_flags(int argc, char** argv)
{
    std::vector<FlagSet> out;

    std::vector<int> short_flag_indexes;
    std::vector<int> long_flag_indexes;

    char* version_flag = "--version";
    char* help_flag = "--help";
    // Check for --version or --help.
    if (argc == 2) {
        if (strcmp(argv[1], version_flag) == 0) {
            std::print("nucleofile v0.1.2\n");
            return out;
        }
        if (strcmp(argv[1], help_flag) == 0) {
            std::print("read the docs.\n");
            return out;
        }
    }

    for (int i = 1; i < argc; i++) {
        if (argv[i][0] == '-') {
            if (argv[i][1] == '-') {
                // Long flag.
                if ( argv[i][2] == '\0') {
                    // Treat all past this as string input to app.
                }
            }
            // Short flag.
            if (argv[i][1] == '\0') {
                // Not a clue what that is. Return error.
            }
        } else {
            // Param for previous flag.
        }
    }

    return out;
}

class Args {
private:
    std::vector<FlagSet> flags;

public:
    Args(int argc, char** argv)
    {
        // Actually parse args here.
        flags = set_flags(argc, argv);
    }
};

int run_app(int argc, char** argv)
{
    int status = 0;

    // Parse args.
    Args args(argc, argv);

    // Get user config file.
    auto config = set_config();

    // Run interpreter or gui with AppState.
    status = Gui::run_gui();

    return status;
}
