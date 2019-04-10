#include <stdio.h>
#include <stdlib.h>
#include "arraylist.h"
#include <string.h>

int out(char* str) {
	printf("%s", str);
	return 1;
}

int out_line(char* str) {

	printf("%s\n", str);
	return 1;
}

int in(char * str) {
	fgets(str, 50, stdin);
	return 1;
}

char* copyVal(char* c) {

  char* ret = malloc(sizeof(c));

  char tmp = ' ';
  for (int i = 0; i < 50; i++) {

    ret[i] = c[i];

    if (c[i] == '\0') 
      break;

  }

  return ret;
}

ArrayList * list_new(unsigned int len) {
	return arraylist_new(len);
}

int list_add(ArrayList* l, char * str) {
	arraylist_append(l, copyVal(str));
	return 1;
}

int set(char* dest, char* src) {
	
	strcpy(dest, src);
	return 1;
}

int sys(char* str) {
	
	int ret = system(str);

	if (ret == 0) {
		return 1;
	}

	return 0;
}
