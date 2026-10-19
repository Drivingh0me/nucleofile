TARGET = nucleofile

SRC_DIR := src
BUILD_DIR = build
BIN_DIR = bin
INCLUDE_DIR = include

# This selects the paltform
ifeq ($(shell uname -s),Darwin)
	PLATFORM := Darwin
	cc := clang++

	GLFW_INCLUDE_DIR := /opt/homebrew/opt/glfw/include
	GLFW_LIB_DIR := /opt/homebrew/opt/glfw/lib
	# CGLM_INCLUDE_DIR := /opt/homebrew/opt/cglm/include/cglm

	CFLAGS = -std=c++23 -Wall -Wextra -I$(INCLUDE_DIR) -I$(GLFW_INCLUDE_DIR)
	LDFLAGS = -L$(GLFW_LIB_DIR) -lglfw -ldl -framework OpenGL -framework Cocoa -framework Foundation
endif

ifeq ($(shell uname -s),Linux)
	PLATFORM := Linux
	cc := g++

	GLFW_INCLUDE_DIR := /opt/homebrew/opt/glfw/include
	GLFW_LIB_DIR := /opt/homebrew/opt/glfw/lib

	CFLAGS = -std=c++23 -Wall -Wextra -I$(INCLUDE_DIR)
	LDFLAGS = -lglfw -ldl -lGl -lm
	# LDFLAGS := -L$(GLFW_LIB_DIR) -lglfw -ldl -lGL -lm
endif

PLATFORM_DIR = src/platform/$(PLATFORM)
# Add a way to compile .mm files.

SRCS := $(wildcard $(SRC_DIR)/*.cpp)
PLAT_SRC := $(wildcard $(PLATFORM_DIR)/*.cpp)

OBJS := $(patsubst $(SRC_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))
PLAT_OBJ := $(patsubst $(PLATFORM_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(PLAT_SRC))

CT_FLAGS = -Wall -Wextra

all: $(TARGET)

$(TARGET): $(OBJS) $(PLAT_OBJ)
	mkdir -p $(BUILD_DIR)/$(BIN_DIR)
	$(cc) $^ $(CT_FLAGS) -o $(BUILD_DIR)/$(BIN_DIR)/$(TARGET) $(LDFLAGS)

$(BUILD_DIR)/%.o: $(PLATFORM_DIR)/%.cpp
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) -o $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) -o $@

clean:
	rm -rf $(BUILD_DIR)
	# rm $(BUILD_DIR)/*.o

# .PHONY: all clean debug
.PHONY: all clean
