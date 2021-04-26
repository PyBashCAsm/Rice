#include <common.h>

char *getcode(FILE *tgt_file){
	if (tgt_file==NULL){
		return NULL;
	}
	char *buff=(char*) malloc(BUFFERSIZE*sizeof(char));
	if (buff==NULL){
		printf("RICE: FATAL INTERNAL ERROR -- EXITING IMMEDIATELY");
		exit(-1);
	}
	int token;
	int counter=0;
	int buffersize=BUFFERSIZE;
	while (1){
		token=getc(tgt_file);
		if (token=='\n'){ 
			buff[counter]=';';
			break;
		}
		else if(token==EOF){
			*buff='#';
			return buff;
		}
		if (counter==BUFFERSIZE){
			buffersize+=buffersize;
			buff=(char*) realloc(buff,buffersize*sizeof(char));
			if (buff==NULL){
				printf("RICE: FATAL INTERNAL ERROR-- EXITING IMMEDIATELY");
				exit(-1);
		}
		}
		buff[counter]=token;
		counter++;
		}
	return buff;
}
