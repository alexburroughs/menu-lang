#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include "std.h"
#include "arraylist.h"
static int SIZE_BUFF = 50;
int main(int argc, char** argv) {char* a = malloc(sizeof(char) * SIZE_BUFF);
char* b = malloc(sizeof(char) * SIZE_BUFF);
char* c = malloc(sizeof(char) * SIZE_BUFF);
char* var = malloc(sizeof(char) * SIZE_BUFF);
out ("Welcome to the demo\n");
set (var,"var");
out ("setting a variable test: ");
out_line (var);
out_line ("Enter a word");
int result = in (a);
if ( result ) {
out ("success\n");
out ("your word is: ");
out_line (a);
out_line ("\nEnter another word");
in (var);
}
if ( result ) {
out_line ("testing results");
}
out ("testing ");
out_line ("semicolons\n");
out_line ("\nYour word called from a function pointer is: ");
int  (* pnt ) (char*) = out_line;
pnt (var);
out_line ("Enter a system command");
in (var);
int sys_out = sys (var);
while ( sys_out ) {
out_line ("Enter another command:");
in (var);
sys_out = sys (var);
}
out_line ("exiting...");
free(a);
free(b);
free(c);
free(var);
}