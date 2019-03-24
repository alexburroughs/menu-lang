#include <stdio.h>
#include <stdlib.h>

int out(char* str) {
	printf("%s", str);
	return 1;
}

int in(char * str) {
	scanf("%s", &str);
	return 1;
}
