#include "str.h"
#include <stdio.h>
#include <stdlib.h>

char* prompt(const char* prompt){
	printf("%s",prompt);
	char *input=getstr();                                    
	return input;
}

int cpy(char *dest_buff,const char *target_buff){
	if(dest_buff==NULL || target_buff==NULL){
		return -1;
	}
	int counter=0;
	while (1){
		if (len(target_buff)<counter){
			break;
		}
		dest_buff[counter]=target_buff[counter];
		counter++;
	}
	return counter;
}

int getindex(const char *buffer,const char target){
	if (buffer==NULL){
		return -1;
	}
	int counter=0;
	while(1){
		if (counter+1==len(buffer)){
			if (buffer[counter]==target){
			break;
			}
			else return -1;
		}
		if (buffer[counter]==target){
			break;
		}
		counter++;
	}
	return counter;
}

char *getsubstring(const char *buffer,char start,char end){
	int start_index=(getindex(buffer,start)+1);
	int end_index=getindex(buffer,end);
	int counter=0;
	char *substring=(char*) malloc(((end_index-start_index)+1)*sizeof(char));
	if (start_index==-1 || end_index==-1){
		return NULL;
	}
	while (1){
		if(start_index==end_index){
			break;
		}
		substring[counter]=buffer[start_index];
		counter++;
		start_index++;
	}
	return substring;
}

