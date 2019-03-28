#include <stdio.h>
#include <stdlib.h>
#include <arraylist.h>

int out(char* str) {
	printf("%s", str);
	return 1;
}

int in(char * str) {
	scanf("%s", &str);
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
