IDIR =../inc
CC=gcc
CFLAGS=-I$(IDIR)

ODIR=obj

_DEPS = newtons_method.h implicit_runge_kutta.h
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

_OBJ = main.o newtons_method.o implicit_runge_kutta.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

a.out: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
