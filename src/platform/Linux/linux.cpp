#include "platform_specific.h"
#include <print>

Config set_config() {
    Config config;
    config.color_mode = 0;

    std::print("Config set! \n");

    return config;
}

// All platform specifc calls for linux.
