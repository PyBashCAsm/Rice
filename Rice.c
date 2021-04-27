#include <common.h>

int main(){
	FILE *input=fopen("dev.ri","r");
	char *buff=NULL;
	int line=1;
	do {
		buff=getcode(input);
		if (buff[0]=='#') break;
		error_reporter("dev.ri",buff,line);
		line++;
		
	} while(1);
	fclose(input);

	return 0;
}
