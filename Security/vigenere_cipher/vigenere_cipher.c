#include<stdio.h>
#include<string.h>
#include<ctype.h>
#include<stdlib.h>

#define NLET 26 // number of letters
#define UTL 32 // upper to lower

typedef char *string;

// frees buffer and sets it to NULL
void dealloc(string* buffer) {
    free(*buffer);
    *buffer = NULL;
}

// Function to take user input, prevents buffer overflow (;
string input() {

    int space = 24;
    string buffer = (string)malloc(space);
    
    if(buffer == NULL) { 
        dealloc(&buffer); 
        exit(EXIT_FAILURE);
    };

    int ch = EOF, cursor = 0;

    while((ch = getchar()) != '\n' && ch != EOF) { 
        
        buffer[cursor++] = (char)ch;

        /* if cursor crosses current buffer space
            it's doubled in space and new space gets reallocated */
        if(cursor >= space - 1) { 
            space <<=1;
            buffer = (string)realloc(buffer, space);
			if(buffer == NULL) {
                dealloc(&buffer); 
                exit(EXIT_FAILURE);
            };
        }
        
    }
    buffer[cursor] = '\0'; // null termination

    // Minimize buffer
    buffer = (string)realloc(buffer, strlen(buffer)+1);
    
    return buffer;
}

string key_generator(string key, string cred){

    size_t cred_len, key_len, j, i;

    cred_len = strlen(cred);
    key_len = strlen(key);

    string valid_key = (string)malloc(sizeof(char) * key_len+1); 
    string gen_key = (string)malloc(sizeof(char) * cred_len+1);

    if(gen_key == NULL || valid_key == NULL){
        dealloc(&key); dealloc(&gen_key);
        dealloc(&valid_key);
        exit(EXIT_FAILURE);
    }

    // append alphabetic characters in valid_key
    for(i = 0, j = 0; i < key_len; ++i){
        if(isalpha(key[i]))
            valid_key[j++] = key[i];
    }
    // checks for empty value
    if(!strcmp(valid_key, "")){
        strcpy(gen_key, valid_key);
        goto ret;
    }
    // generating key with user provided key
    key_len = strlen(valid_key);
    for(i = 0, j = 0; i < cred_len; ++i){

        if(j >= key_len) 
            j = 0;

        if(!isalpha(i[cred])){
            gen_key[i] = cred[i];
        } else { // if non-alphabetic character, gen_key is appended with keys' current element
            gen_key[i] = valid_key[j++]; 
        }
    }
    gen_key[i] = '\0';   

    ret:
        dealloc(&key); dealloc(&valid_key);
        return gen_key;
}


void eval(string cred, string key, string mode){

    size_t cred_len = strlen(cred), i;
    char *res = (string)malloc(sizeof(char) * cred_len+1);

    if(res == NULL){
        goto deallocation;
    }
    // checks for empty key and data
    if(!strcmp(key, "") || !strcmp(cred, "")) {
        puts(cred);
        goto deallocation;
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
        res[i] = '\0'; // null terminator

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
        res[i] = '\0'; // null terminator
    } else { // if mode is not encrypt or decrypt throw error;
        fprintf(stderr, "\033[31mModeError: %s is not a valid mode\033[0m\n", mode);
        goto deallocation;
    }
    puts(res);

    deallocation: 
        dealloc(&mode); dealloc(&cred); dealloc(&res); dealloc(&key);
}

int main(void) {

    fprintf(stdout, "\033[32mEnter the mode you want to perform\033[0m: ");
    string mode = input();
    fprintf(stdout, "\033[33mEnter the data string you want to %s\033[0m: ", mode);
    string data = input();
    fprintf(stdout, "\033[36mEnter the key you want your data to %s with\033[0m: ", mode);
    string key = input();
    string genKey = key_generator(key, data);
    eval(data, genKey, mode);

    return 0;
}
