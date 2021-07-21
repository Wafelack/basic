CC := c89
CFLAGS := -Wall -Wextra -Werror -pedantic
SRCS := $(shell find ./ -name "*.c")
OBJS := $(SRCS:.c=.o)

all: $(OBJS)
	$(CC) $^ -o basic
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
clean:
	rm -rf $(OBJS)
	rm -f basic
format:
	clang-format --style=GNU -i $(SRCS) $(shell find ./ -name "*.h")
.PHONY: clean
