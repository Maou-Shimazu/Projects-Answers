#include<stdio.h>
#include<string.h>
#include<ctype.h>
#include<stdlib.h>

#define NLET 26 // number of letters
#define UTL 32 // upper to lower

// Function to delete element at the given index from string 
int purge(char*data, int idx, int len){

    int i;
    for (i = idx; i < (len)-1; i++)
        *(data+i) = *(data+i+1);

    *(data+i) = '\0';
    return --len;

}

void dealloc(char** buffer) {
    free(*buffer);
    *buffer = NULL;
}

// Function to take user input, prevents buffer overflow (;
char* input() {

    int buffsize = 256;
    char* buffer = (char*)malloc(buffsize);
    
    if(buffer == NULL) exit(EXIT_FAILURE);

    int ch = EOF, cursor = 0;

    while((ch = getchar()) != '\n' && ch != EOF) { 
        
        buffer[cursor++] = (char)ch;

        /* if cursor crosses current buffer size
            it's doubled in size and new size gets reallocated */
        if(cursor >= buffsize - 1) { 
            buffsize <<=1;
            buffer = (char*)realloc(buffer, buffsize);
			if(buffer == NULL) exit(EXIT_FAILURE);
        }
        
    }
    buffer[cursor] = '\0';
    
    return buffer;
}

char* key_generator(char*key, char*cred){

    size_t cred_len, key_len, j = 0, i;

    cred_len = strlen(cred);
    key_len = strlen(key);

    char *gen_key = (char*)malloc(sizeof(char) * cred_len+1);

    if(gen_key == NULL)
        exit(1);

    // removes non-alphabetic character from key :>
    for(i = 0; i < key_len; ++i){
        while(isalpha(key[i]) == 0)
            key_len = purge(key, i, key_len);
    }
    // generating key with user provided key
    for(i = 0; i < cred_len; ++i){

        if(j >= key_len) 
            j = 0;

        if(!isalpha(i[cred])){
            gen_key[i] = cred[i];
        } else { // if non-alphabetic character, gen_key is appended with keys' current element
            gen_key[i] = key[j]; 
            ++j;
        }
    }
    gen_key[i] = '\0';   
    dealloc(&key);
    return gen_key;
}


void eval(char* cred, char* key, char* mode){

    size_t cred_len = strlen(cred), i;
    char *res = (char*)malloc(sizeof(char) * cred_len+1);

    if(res == NULL)
        exit(1);

    if(!strcmp(mode, "encrypt")){
        for(i = 0; i < cred_len; ++i){

            // checks if cred element at index i is between A or Z
            if((toupper(cred[i]) >=65 && toupper(cred[i]) <=90)){
                //res is appended with encrypted char
                res[i] = toupper(cred[i]) - (65 - toupper(key[i]));

                // checks if the encrypted char is greater than Z
                if(res[i] > 90)
                    //encrypted character is subtracted with 26, so it's between A and Z
                    res[i] = i[res]-NLET;
            } else // if element is a non-alphabetic character, res is appended with the element
                res[i] = cred[i];

            // matching case in both data and encrypted data
            if(islower(cred[i])){
                res[i] = res[i]+UTL;
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
                    res[i] = i[res]+NLET;
            }else //else decryptedCache is appended with creds' current element
                res[i] = cred[i];

            // matching case in both encrypted and decrypted data
            if(islower(cred[i])){
                res[i] = res[i]+UTL;
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
    char* mode = input();
    fprintf(stdout, "\033[33mEnter the data string you want to %s\033[0m: ", mode);
    char* data = input();
    fprintf(stdout, "\033[36mEnter the key you want your data to %s with\033[0m: ", mode);
    char* key = input();
    char* genKey = key_generator(key, data);
    eval(data, genKey, mode);

return 0;
}
