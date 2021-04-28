#include "common.h"

void error_reporter(const char* filename, const char* line,int line_num){
	int length=len(line);
	int validity=keyword_type(line);
	//printf("%d\n",validity);
	if (cmp(line,";")==EQUAL) return;
	switch(validity){
		case ILLEGAL_KEYWORD:
			printf("RICE:%s:%d Illegal keyword '%s' (Add ending angled brackets '>')\n",filename,line_num,getsubstring(line,'<',';'));
			break;
		case UNKNOWN_KEYWORD:
				printf("RICE:%s:%d The keyword '%s' is not valid\n",filename,line_num,getsubstring(line,'<',';'));
	}
}

