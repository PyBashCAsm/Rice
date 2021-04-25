#include <common.h>

int main(){
	FILE *input=fopen("dev.ri","r");
	char *buff=NULL;
	int line=0;
	do {
		buff=getcode(input);
		if (buff[0]=='#') break;
		int type=keyword_type(buff);
		switch(type){
			case START:
				printf("Prog start\n");
				break;
			case END: printf("Prog end\n");
				  break;
			case DEFINE_FUNC:printf("Function was defined\n");
					 break;
			case END_FUNC_DEF:printf("Function definition ends");
			}
	} while(1);

	return 0;
}
