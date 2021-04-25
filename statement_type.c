#include <common.h>

int keyword_type(const char *line){
	char start='<',end='>';
	char *keywords[]={"start","end","DEF_FUNC","END_FUNC_DEF"};
	if (line[0]!=start) return NOT_KEYWORD;
	else 
		if(getindex(line,end)==-1) return ILLEGAL_KEYWORD;
	char *keyword;
	keyword=getsubstring(line,start,end);
	int counter=0;
	while(counter<=3){
		if(cmp(keyword,keywords[counter])==EQUAL) break;
		counter++;
	}
	switch(counter){
		case 0:return START;
		case 1:return END;
		case 2:return DEFINE_FUNC;
		case 3:return END_FUNC_DEF;
	}
	return UNKNOWN_KEYWORD;
}
