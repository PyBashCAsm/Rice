#include <stdio.h>
#include <stdlib.h>
#include "str.h"

#define DEFAULT_BUFFSIZE 1024

char*  getstr(){
	char *buffer=(char*) malloc(DEFAULT_BUFFSIZE*sizeof(char));
	int buffer_len=DEFAULT_BUFFSIZE;
	int letter,index=0;
	while (1){
		letter=getc(stdin);
		if (letter==EOF || letter=='\n'){
			buffer[index]='\0';
			break;
		}
		if (index==buffer_len){
			buffer_len+=buffer_len;
			buffer=(char*) realloc(buffer,buffer_len);
		}
		buffer[index]=letter;
		index++;
	}
	return buffer;
}

int len(const char *buffer){
	if (buffer==NULL) return -1;
	else if(buffer[0]=='\0') return 0;
	char letter;
	int len;
	for (len=1;;len++){
		letter=buffer[len];
		if (letter=='\0'){
			break;
		}
	}
	return len;
}


int cmp(const char *string1,const char *string2){
	if (len(string1)!=len(string2)){
		return NOT_EQUAL;
	}
	int count=0;
	while (1){
		if (count==len(string1)){
			break;
		}
		if (string1[count]!=string2[count]){
			return NOT_EQUAL;
		}
		count++;
	}
	return EQUAL;
}


