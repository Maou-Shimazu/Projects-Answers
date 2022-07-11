#include<stdio.h>
#include<string.h>
#include<stdlib.h>

static inline void* allocate(size_t _Count, size_t _Size) {
    void* tmp = calloc(_Count, _Size);
    if(tmp == NULL) 
        exit(EXIT_FAILURE);
    return tmp;
}

static inline void* reallocate(void* _Buffer, size_t _Size) {
    void* tmp = realloc(_Buffer, _Size);
    if(tmp == NULL){ 
        free(_Buffer);
        exit(EXIT_FAILURE);
    }
    return tmp;
}

/*
 char* var = readline(_Stream); 
 // stream can be stdin or a file pointer that has already been opened
 _____________________________________________________________________
 readline() reads char values from given stream until it detects LF
*/
char* input(char* _Prompt) {

    printf("%s", _Prompt);

    size_t space = 80, cursor = 0;
    char* buffer = (char*)allocate(1, space);

    int ch = EOF;
    while((ch = getchar()) != '\n' && ch != EOF) { 
        
        buffer[cursor++] = (char)ch;

        /* if cursor crosses current buffer space
            it's doubled in space and new space gets reallocated */
        if(cursor >= space - 1) { 
            space <<=1;
            buffer = (char*)reallocate(buffer, space);
        }
    }

    buffer[cursor] = '\0'; // null termination

    // Minimize buffer
    buffer = (char*)reallocate(
        buffer, 
        strlen(buffer)+1
    );

    return buffer;
}


void reverse(char*str){
   
   int i = 0, j = (int)strlen(str)-1;
   while(i < j){
      int temp  = *(str+i);
      *(str+i++) = *(str+j);
      *(str+j--) = (char)temp;
   }
}

int main(void) {

    char* string = input(": ");
    reverse(string);
    puts(string);
    free(string);
    return 0;
}
