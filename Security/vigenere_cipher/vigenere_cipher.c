#include<stdio.h>
#include<string.h>
#include<ctype.h>
#include<stdlib.h>

#define NLET 26 // number of letters
#define UTL 32 // upper to lower


// typedefing char pointer to string for readability
typedef char *string;

static size_t allocations = 0;

// vault will store all allocated memory on vault
static string *vault = NULL;

// to add allocated memory into vault
static void push(string mem) {

    // Resize array so as to append string
    string *temp = realloc(vault, sizeof(string) * (allocations+1));
    if (temp == NULL) {
        free(mem);
        return;
    }
    vault = temp;

    // Append string to array
    vault[allocations++] = mem;
}

// deallocates memory on the heap iteratively
static void dalloc(void) {

    if(vault != NULL) {

        for(size_t idx = 0; idx < allocations; ++idx) {
            free(vault[idx]);
        }
        free(vault);
    }
}

// Function to take user input, prevents buffer overflow (;
string input(void) {

    size_t space = 5, cursor = 0;
    string buffer = (string)malloc(space);
    
    if(buffer == NULL) { 
        free(buffer);
        exit(EXIT_FAILURE);
    };

    int ch = EOF;

    while((ch = getchar()) != '\n' && ch != EOF) { 
        
        buffer[cursor++] = (char)ch;

        /* if cursor crosses current buffer space
            it's doubled in space and new space gets reallocated */
        if(cursor >= space - 1) { 
            space <<=1;
            buffer = (string)realloc(buffer, space);
			if(buffer == NULL) {
                free(buffer);
                exit(EXIT_FAILURE);
            }
        }
        
    }

    buffer[cursor] = '\0'; // null termination

    // Minimize buffer
    buffer = (string)realloc(buffer, strlen(buffer)+1);

    // pushing allocated memory into vault
    push(buffer);

    return buffer;
}

string generateKey(string key, string cred){

    size_t cred_len, key_len, j, i;

    cred_len = strlen(cred);
    key_len = strlen(key);

    string gen_key = (string)malloc(sizeof(char) * cred_len+1);

    if(gen_key == NULL){
        free(gen_key);
        exit(EXIT_FAILURE);
    }

    // append alphabetic characters in key
    for(i = 0, j = 0; i < key_len; ++i){
        if(isalpha(key[i]))
            key[j++] = key[i];
    }
    key[j] = '\0';

    // checks for empty value
    if(!strcmp(key, "")){
        strcpy(gen_key, key);
        goto ret;
    }

    key_len = strlen(key);
    
    // generating key with user provided key
    for(i = 0, j = 0; i < cred_len; ++i){

        if(j >= key_len) 
            j = 0;

        if(!isalpha(i[cred])){
            gen_key[i] = cred[i];
        } else { // if non-alphabetic character, gen_key is appended with keys' current element
            gen_key[i] = key[j++]; 
        }
    }
    gen_key[i] = '\0'; // null termination  

    ret:
        push(gen_key);
        return gen_key;
}


void eval(string cred, string key, string mode){

    size_t cred_len = strlen(cred), i;
    string res = (string)malloc(sizeof(char) * cred_len+1);

    if(res == NULL){
        free(res);
        exit(EXIT_FAILURE);
    }

    if(!strcmp(mode, "encrypt")){
        for(i = 0; i < cred_len; ++i){

            // checks if cred element at index i is between A or Z
            if((toupper(cred[i]) >=65 && toupper(cred[i]) <=90)){
                //res is appended with encrypted char
                res[i] = toupper(cred[i]) - (65 - toupper(key[i]));

                // checks if the encrypted char is greater than Z
                if(res[i] > 90)
                    //encrypted character is subtracted with 26, so it's between A and Z
                    res[i] -= NLET;
            } else // if element is a non-alphabetic character, res is appended with the element
                res[i] = cred[i];

            // matching case in both data and encrypted data
            if(islower(cred[i])){
                res[i] += UTL;
            }
        }
        res[i] = '\0'; // null termination

    } else if(!strcmp(mode, "decrypt")){

        for(i = 0; i < cred_len; ++i){
            
            // checks if cred element at index i is between A or Z
            if((toupper(cred[i]) >=65 && toupper(cred[i]) <=90)){
                //res is appended with decrypted char
                res[i] = toupper(cred[i]) - (toupper(key[i]) - 65);
                // checks if the decrypted char is lesser than A
                if(res[i] < 65)
                    //decrypted character is added with 26, so it's between A and Z
                    res[i] += NLET;
            }else //else decryptedCache is appended with creds' current element
                res[i] = cred[i];

            // matching case in both encrypted and decrypted data
            if(islower(cred[i])){
                res[i] += UTL;
            }
        }
        res[i] = '\0'; // null termination
        
    } else { // if mode is not encrypt or decrypt throw error;
        fprintf(stderr, "\033[31mModeError: %s is not a valid mode\033[0m\n", mode);
        goto exec;
    }

    // checks for empty key and data
    if(!strcmp(key, "") || !strcmp(cred, "")) {
        strcpy(res, cred);
    }

    puts(res);
    exec:
        push(res);
}

int main(void) {

    // call dalloc before exiting the program to free memory allocated on heap
    atexit(dalloc);

    fprintf(stdout, "\033[32mEnter the mode you want to perform\033[0m: ");
    string mode = input();
    fprintf(stdout, "\033[33mEnter the data string you want to %s\033[0m: ", mode);
    string data = input();
    fprintf(stdout, "\033[36mEnter the key you want your data to %s with\033[0m: ", mode);
    string key = input();
    string genKey = generateKey(key, data);
    eval(data, genKey, mode);

    return 0;
}
