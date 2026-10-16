#include <iostream>

#include "app.hpp"
#include "gui.hpp"
#include "platform_specific.h"

class Args {
private:
    int num_of_args;

public:
    Args(int argc, char** argv) {
        // Actually parse args here.
        num_of_args = argc;
    }
    ~Args() {
        // Destructor.
    }
};

int run_app(int argc, char** argv) {
    int status = 0;

    // Parse args.
    Args args(argc, argv);

    // Get user config file.
    auto config = set_config();

    // Run interpreter or gui with AppState.
    status = Gui::run_gui();

    return status;
}
