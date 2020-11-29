IDIR = inc
SDIR = src
ODIR = build

CC = gcc
CFLAGS = -I$(IDIR)

SRC := $(wildcard $(SDIR)/*.c)
OBJ := $(patsubst $(SDIR)/%.c,$(ODIR)/%.o,$(SRC))

output: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)

$(ODIR)/%.o: $(SDIR)/%.c $(IDIR)/%.h
	$(CC) -c -o $@ $< $(CFLAGS)