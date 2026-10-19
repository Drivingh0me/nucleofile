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

	MMFLAGS = -x objective-c++

	LDFLAGS = -L$(GLFW_LIB_DIR) -lglfw -ldl -framework OpenGL -framework Cocoa -framework Foundation
endif

ifeq ($(shell uname -s),Linux)
	PLATFORM := Linux
	cc := g++

	CFLAGS = -std=c++23 -Wall -Wextra -I$(INCLUDE_DIR)
	LDFLAGS = -lglfw -ldl -lGL -lm
endif

PLATFORM_DIR = src/platform/$(PLATFORM)
# Add a way to compile .mm files.

SRCS := $(wildcard $(SRC_DIR)/*.cpp)
PLAT_SRC_CPP := $(wildcard $(PLATFORM_DIR)/*.cpp)
PLAT_SRC_MM := $(wildcard $(PLATFORM_DIR)/*.mm)
PLAT_SRC := $(PLAT_SRC_CPP) $(PLAT_SRC_MM)

OBJS := $(patsubst $(SRC_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))
PLAT_OBJ_CPP := $(patsubst $(PLATFORM_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(PLAT_SRC_CPP))
PLAT_OBJ_MM := $(patsubst $(PLATFORM_DIR)/%.mm, $(BUILD_DIR)/%.o, $(PLAT_SRC_MM))
PLAT_OBJ := $(PLAT_OBJ_CPP) $(PLAT_OBJ_MM)

CT_FLAGS = -Wall -Wextra

all: $(TARGET)

$(TARGET): $(OBJS) $(PLAT_OBJ)
	mkdir -p $(BUILD_DIR)/$(BIN_DIR)
	$(cc) $^ $(CT_FLAGS) -o $(BUILD_DIR)/$(BIN_DIR)/$(TARGET) $(LDFLAGS)

$(BUILD_DIR)/%.o: $(PLATFORM_DIR)/%.cpp
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) -o $@

$(BUILD_DIR)/%.o: $(PLATFORM_DIR)/%.mm
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) $(MMFLAGS) -o $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	mkdir -p $(BUILD_DIR)
	$(cc) -c $< $(CFLAGS) -o $@

clean:
	rm -rf $(BUILD_DIR)
	# rm $(BUILD_DIR)/*.o

# .PHONY: all clean debug
.PHONY: all clean
