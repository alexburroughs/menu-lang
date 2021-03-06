#include "arraylist.h"

#ifndef STD_H
#define STD_H
int out(char* str);
int out_line(char* str);
int in(char* str);
int set(char* dest, char* src);
int list_add(ArrayList *l, char* str);
int sys(char* str);
int expect(char* ex, char* msg, int actual);
int concat(char* num, char* onto,...);
ArrayList * list_new(unsigned int tlen);
#endif
