#include <stdlib.h>

typedef struct func_node {
	char * val;
	struct func_node * next;
} F_NODE;

typedef struct func_arg {
	F_NODE * head;
	F_NODE * curr;
	unsigned short size;
} F_ARG;

F_ARG * _INIT_ARG() {
	F_ARG * ret = malloc(sizeof(F_ARG));
	ret->size = 0;
	return ret;
}

void _ADD_ARG(F_ARG * list, char * add) {
	
	F_NODE * tmp = malloc(sizeof(F_NODE));
	tmp->val = add;	

	if (list->size == 0) {
		list->head = tmp;
		list->curr = tmp;
		list->size++;
	}
	else {
		list->curr->next = tmp;
		list->size++;
	}
}

void _FREE_LIST(F_ARG * list) {
	
	F_NODE * curr;
	F_NODE * next;

	curr = list->head;
	
	while (curr != NULL) {
		next = curr->next;
		free(curr);
		curr = next;
	}

	free(list);
}
