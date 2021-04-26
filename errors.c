#include <common.h>

void parse(const char* filename, const char* line,int line_num){
	int length=len(line);
	int validity=keyword_type(line);
	//printf("%d\n",validity);
	if (validity==NOT_KEYWORD ||  cmp(line,";")==EQUAL) return;
	else if(validity==ILLEGAL_KEYWORD){
		printf("RICE:%s:%d Illegal keyword '%s' (Add ending angled brackets '>')\n",filename,line_num,getsubstring(line,'<',';'));
	}
}
