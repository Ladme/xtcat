xtcat: main.c
	gcc main.c -o xtcat -std=c99 -pedantic -Wall -Wextra -O3 -march=native

install: xtcat
	cp xtcat ${HOME}/.local/bin
