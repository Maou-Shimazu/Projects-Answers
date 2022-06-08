
BIN=gcc
CFLAGS=-g -Wall

all: bs64

base64.o: base64.c
	$(BIN) $(CFLAGS) -c $^

utils.o: utils.c utils.h
	$(BIN) $(CFLAGS) -c $^

bs64: base64.o utils.o
	$(BIN) $(CFLAGS) $^ -o $@

clean:

ifeq ($(OS),Windows_NT)
	del *.o bs64* *.gch 
else
	rm *.o bs64* *.gch
endif