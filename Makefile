target:str.c str2.c str.h getline.c keyword_detect.c Rice.c errors.c
	gcc -c -fPIC str.c str2.c 
	gcc -shared str.c str2.c -o libstr.so
	export LD_LIBRARY_PATH=./
	gcc getline.c keyword_detect.c Rice.c errors.c -L. -lstr  -o Rice

clean:
	rm *.o
