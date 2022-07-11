
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>


// allocations will store number of allocations
static size_t allocations = 0;

// vault will store all allocated memory 
static char** vault = NULL;

/*
 push(memloc) | a static function which pushes allocated 
 memory to vault which will be deallocated before 
 program exits
*/
static inline void push(void* memloc) {

    // Resize array so as to append char*
    char**temp = (char**)realloc(vault, sizeof(char*) * (allocations+1));
    if (temp == NULL) {
        free(memloc);
        return;
    }
    vault = temp;

    // Append char* to array
    vault[allocations++] = memloc;
}

/*
 dealloc() | a static function which free all the 
 allocated memory stored in vault
*/
static void dealloc(void) {

    if(vault != NULL) {

        for(size_t idx = 0; idx < allocations; ++idx) {
            free(vault[idx]);
        }
        free(vault);
    }
}


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

    push(buffer);
    return buffer;
}


/*
char* var = replace(_String, _Substring, _Substitute)
_______________________________________________________
replace() replaces all instances of _Substring from 
_String with _Substitute
*/
char* replace(char* _String, char* _Substring, char* _Substitute) {

    char* res = NULL, *ptr = _String;
    size_t _Sub_len = strlen(_Substitute), 
           _Substr_len = strlen(_Substring),
           _Length = strlen(_String);

    if(!strcmp(_Substring, "")){
        res = (char*)allocate(
            1, 
            _Length + (_Length*_Sub_len) + 1
        );

        strcat(res, _Substitute);
        for(size_t idx = 0; idx < _Length; idx++) {
            strncat(res, _String+idx, 1);
            strcat(res, _Substitute);
        }
    }

    else{
        res = strdup(_String);
        while ((ptr=strstr(ptr,_Substring))!=NULL) {
            
            size_t skip = strlen(res) - strlen(ptr), eval = strlen(ptr+_Substr_len);
            res = (char*)reallocate(res, skip+_Sub_len+eval+1);
            strcpy(res+skip, _Substitute);
            strncat(res, ptr+_Substr_len, eval);
            ptr++; 
        }
    }
    push(res);
    return res;
}

int main(void) {

    atexit(dealloc);
    char* string = input("Enter the string: ");
    char* substring = input("Enter the substring: ");
    char* substitute = input("Enter the substitute: ");
    puts(replace(string, substring, substitute));
}

