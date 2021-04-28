#ifndef __LIBSTR_HEADER__
#define __LIBSTR_HEADER__

#define EQUAL 0
#define NOT_EQUAL 1

extern char* getstr();
extern int cmp(const char* string1,const char* string2);
extern int len(const char* buffer);
extern char* prompt(const char* prompt);
extern int cpy(char *dest_buff,const char *target_buff);
extern int getindex(const char *buffer,const char target);
extern char *getsubstring(const char *buffer,char start,char end);

#endif
