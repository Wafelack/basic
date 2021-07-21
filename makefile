CC := c89
CFLAGS := -Wall -Wextra -Wwrite-strings -Werror=discarded-qualifiers -pedantic
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
release: CFLAGS+=-Werror -O3
release: all
.PHONY: clean
