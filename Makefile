TARGET = nucleofile

SRC_DIR := src
BUILD_DIR = build
BIN_DIR = bin
INCLUDE_DIR = include

# This selects the paltform
ifeq ($(shell uname -s),Darwin)
	PLATFORM := Darwin
	cc := clang++
endif

ifeq ($(shell uname -s),Linux)
	PLATFORM := Linux
	cc := g++
endif

PLATFORM_DIR = src/platform/$(PLATFORM)

SRCS := $(wildcard $(SRC_DIR)/*.c)
PLAT_SRC := $(wildcard $(PLATFORM_DIR)/*.c)
# $(info $$SRCS is [${SRCS}])

# OBJS = $(SRCS:.c=.o)
OBJS := $(patsubst $(SRC_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))
PLAT_OBJ := $(patsubst $(PLATFORM_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(PLAT_SRC))

CFLAGS = -Wall -Wextra -I$(INCLUDE_DIR)
CT_FLAGS = -Wall -Wextra
LDFLAGS =

all: $(TARGET)

$(TARGET): $(OBJS) $(PLAT_OBJ)
	$(CC) $^ $(CT_FLAGS) -o $(BUILD_DIR)/$(TARGET)

$(BUILD_DIR)/%.o: $(PLATFORM_DIR)/%.c
	mkdir -p $(BUILD_DIR)
	$(CC) -c $< $(CFLAGS) -o $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	mkdir -p $(BUILD_DIR)
	$(CC) -c $< $(CFLAGS) -o $@

# debug: CFLAGS = -Wall -Wextra -I$(INCLUDE_DIR) -g -O0
# debug: clean $(TARGET)

clean:
	rm -rf $(BUILD_DIR)
	# rm $(BUILD_DIR)/*.o

# .PHONY: all clean debug
.PHONY: all clean
