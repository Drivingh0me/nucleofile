#include <iostream>
#include <string>
#include <vector>

#include "app.hpp"
#include "gui.hpp"
#include "platform_specific.h"

// Flag list:
// -T/--Test = Test the app to see performance and stability.

enum class Flag {
    Test,
    Silent
};

struct FlagSet {
    Flag flag;
    std::vector<std::string> params;
};

std::vector<FlagSet> set_flags(int argc, char** argv)
{
    std::vector<int> short_flag_indexes;
    std::vector<int> long_flag_indexes;
    // Look for short flags of short or long.
    for (int i = 0; i < argc; i++) {
        if (argv[argc][0] = '-') {
            if (argv[argc][1] == '-') {
                // Add its position to list of long flags.
            }
            // Add it's position to list of short flags.
        }
    }

    // Get the number of total flags and use thier positions to determine
    // how many params there are for each flag.

    // Convert the flags to ordered Flags and convert params to Strings.
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
