# Variables
# Compiler stuff
CC=gcc
CFLAGS= -Wall  -Werror -std=c99 -D_XOPEN_SOURCE=500 -D_DEFAULT_SOURCE -Wno-unused-variable -Wno-unused-function

default:
	${CC} ${CFLAGS} src/main.c -o main.out
