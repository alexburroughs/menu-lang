#include <stdlib.h>
#include <stdio.h>
#include "std.h"
static int SIZE_BUFF = 50;
int main() {char* a = malloc(sizeof(char) * SIZE_BUFF);
char* b = malloc(sizeof(char) * SIZE_BUFF);
char* c = malloc(sizeof(char) * SIZE_BUFF);
char* var = malloc(sizeof(char) * SIZE_BUFF);
out ("hello there general kenobi\n");
out (var);
int result = in (a);
if ( result ) {
out ("success");
in (var);
}
out ("here\n");
out ("here\n");
out ("there\n");
out ("over there");
free(a);
free(b);
free(c);
free(var);
}