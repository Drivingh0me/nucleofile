TARGET = nucleofile

SRC_DIR := src
BUILD_DIR = build
BIN_DIR = bin
INCLUDE_DIR = include

# Select the platform
ifeq ($(shell uname -s),Darwin)
	PLATFORM := Darwin
	cc := clang++       # C++ compiler
	c_cc := clang     # C compiler for glad.c

	GLFW_INCLUDE_DIR := /opt/homebrew/opt/glfw/include
	GLFW_LIB_DIR := /opt/homebrew/opt/glfw/lib

    # C++ flags (includes C++23)
	CFLAGS = -std=c++23 -Wall -Wextra -I$(INCLUDE_DIR) -I$(GLFW_INCLUDE_DIR)
    # C flags (no C++ standard flag)
	CFLAGS_C = -Wall -Wextra -I$(INCLUDE_DIR) -I$(GLFW_INCLUDE_DIR)

	MMFLAGS = -x objective-c++
	LDFLAGS = -L$(GLFW_LIB_DIR) -lglfw -ldl -framework OpenGL -framework Cocoa -framework Foundation
endif

ifeq ($(shell uname -s),Linux)
	PLATFORM := Linux
	cc := g++         # C++ compiler
	c_cc := gcc       # C compiler for glad.c

	CFLAGS = -std=c++23 -Wall -Wextra -I$(INCLUDE_DIR)
	CFLAGS_C = -Wall -Wextra -I$(INCLUDE_DIR)
	LDFLAGS = -lglfw -ldl -lGL -lm
endif

PLATFORM_DIR = src/platform/$(PLATFORM)

# Find all source file types
CPP_SRCS := $(wildcard $(SRC_DIR)/*.cpp)
C_SRCS   := $(wildcard $(SRC_DIR)/*.c)
PLAT_SRCS_CPP := $(wildcard $(PLATFORM_DIR)/*.cpp)
PLAT_SRCS_MM  := $(wildcard $(PLATFORM_DIR)/*.mm)

# Generate object file mappings
CPP_OBJS := $(patsubst $(SRC_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(CPP_SRCS))
C_OBJS := $(patsubst $(SRC_DIR)/%.c, $(BUILD_DIR)/%.o, $(C_SRCS))
PLAT_OBJ_CPP := $(patsubst $(PLATFORM_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(PLAT_SRCS_CPP))
PLAT_OBJ_MM := $(patsubst $(PLATFORM_DIR)/%.mm, $(BUILD_DIR)/%.o, $(PLAT_SRCS_MM))

OBJS := $(CPP_OBJS) $(C_OBJS)
PLAT_OBJ := $(PLAT_OBJ_CPP) $(PLAT_OBJ_MM)

CT_FLAGS = -Wall -Wextra

all: $(TARGET)

$(TARGET): $(OBJS) $(PLAT_OBJ)
	mkdir -p $(BUILD_DIR)/$(BIN_DIR)
	$(cc) $^ $(CT_FLAGS) -o $(BUILD_DIR)/$(BIN_DIR)/$(TARGET) $(LDFLAGS)

# Rule for C source files (like glad.c)
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	mkdir -p $(BUILD_DIR)
	$(c_cc) -c $< $(CFLAGS_C) -o $@

# Rule for platform .cpp files
$(BUILD_DIR)/%.o: $(PLATFORM_DIR)/%.cpp
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) -o $@

# Rule for platform .mm files (macOS Objective-C++)
$(BUILD_DIR)/%.o: $(PLATFORM_DIR)/%.mm
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) $(MMFLAGS) -o $@

# Rule for general src .cpp files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) -o $@

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean
