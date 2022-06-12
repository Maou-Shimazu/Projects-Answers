//=================================================_
//      Base64 Encoding and Decoding Tool          |_ 
//         Author: @SynActktraa [Mikey]             |_
//  (Cli Wizard) base64 algorithm implemented in C.  |_
//      © This tool is based on ASCII charset.        |_
//=======================================================


#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "utils.h"


// typedefing char pointer to string for readability
typedef char* string;

size_t allocations = 0;

// vault will store all allocated memory on vault
string *vault = NULL;

// to add allocated memory into vault
void push(string mem) {

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
void dalloc(void) {

    if(vault != NULL) {

        for(size_t idx = 0; idx < allocations; ++idx) {
            free(vault[idx]);
        }
        free(vault);
    }
}

// Function to take user input, prevents buffer overflow (;
string input(string text) {

    size_t space = 16, cursor = 0;
    string buffer = (string)calloc(1, space);
    
    if(buffer == NULL) { 
        free(buffer);
        exit(EXIT_FAILURE);
    };

    int ch = EOF;

    printf("%s", text);
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
    string temp = (string)realloc(buffer, strlen(buffer)+1);
    if(temp == NULL){
        free(buffer);
        exit(EXIT_FAILURE);
    }
    buffer = temp;
    // pushing allocated memory into vault
    push(buffer);

    return buffer;
}


void encode(string plaintext){

    int buffer_len;
    buffer_len = strlen(plaintext);

    // calculate space for base64 encoded string
    int base64_value_space = (0.4*(buffer_len+1))+buffer_len+2;
    // calculate space for binary dump of input string
    int dump_capacity = (buffer_len * 8)+1;

    char six_bit_binary[7], Ox49_val_bin[9]; 

    string base64 = (string)calloc(1, base64_value_space);
    string dump = (string)calloc(1, dump_capacity);

    if(base64 == NULL || dump == NULL) {
        exit(EXIT_FAILURE);
    }

    int i, j, dump_size;

    for( i=0; plaintext[i] != '\0'; ++i ) {
        /*
            charValidate checks for non-ascii characters
        */
            if( charValidate(plaintext[i]) == -1 ){
                puts("InputError: can't take non-ascii characters");
                exit(1);
            }

        string binary = decimal_to_binary(plaintext[i], 8);
        strcpy(Ox49_val_bin, binary);
        free(binary);

        //concatenates the 8 bit binary in dump to create
        //a binary dump which will be manipulated later   
        strcat(dump, Ox49_val_bin);
        memset(Ox49_val_bin, 0x0, strlen(Ox49_val_bin));
    
    }

    dump_size = strlen(dump);
    while( dump_size%6 != 0 )
    /*
        checks if the length of binary dump is in the
        multiplication of 6, coz base64 -> 2^6 = 64
    */
        dump[dump_size++] = 0x30; 

    i = 0, j = 0;  
    while(dump[i] != '\0'){
    /*
        moves 6 bits from dump to six_bit_binary,
        converts the 6 bit binary to decimal and stores
        it in ascii_value and do some comparisions, then
        adds accordingly and stores it in base64 string
        and increments i by 6.
    */
        memset(six_bit_binary, 0x0, strlen(six_bit_binary));
        memmove(six_bit_binary, dump+i, 6);
        six_bit_binary[6] = 0x0;

        int ascii_value = binary_to_decimal(six_bit_binary);
        if( ascii_value >= 0x0 && ascii_value <= 0x19)
            base64[j] = ascii_value + 0x41;
        else if( ascii_value >= 0x1a && ascii_value <= 0x33)
            base64[j] = ascii_value + 0x47;
        else if( ascii_value >= 0x34 && ascii_value <= 0x3d)
            base64[j] = ascii_value - 0x4;
        else if( ascii_value == 0x3e)
            base64[j] = ascii_value - 0x13;
        else if( ascii_value == 0x3f)
            base64[j] = ascii_value - 0x10;
        
        j++; i += 6;

    }
    base64[j] = 0x0;
    free(dump);

    size_t base64_size = strlen(base64); 
    while( base64_size%4 != 0 )
    /*
        inserts '=' at the end of the base64 encoded string until
        the length is in the multiplication of 4.
    */  
        base64[base64_size++] = 0x3d;

    base64[base64_size] = 0x0;

    puts(base64);
    free(base64); 

}


void decode(string base64_data){

	int i, j;
    int buffer_len;

    buffer_len = strlen(base64_data);
    
    // calculates space for base64 encoded string
    int decData_val_space = (buffer_len+2)-(0.15*buffer_len);
    // calculates space for binary dump of input string
    int bin_dump_space = (buffer_len * 6)+1;

    char byte_bin[9];
    string bin_dump = (string)calloc(1, bin_dump_space);
    string decodeData = (string)calloc(1, decData_val_space);
    
    delspace(base64_data);
    buffer_len = strlen(base64_data);
	while(base64_data[buffer_len-1] == 0x3d){
    /* 
        checks for '=' from the end of the input encoded string
        and deletes the padding 
    */
		buffer_len = purge(base64_data, buffer_len-1, buffer_len);
	}


    for( i=0; base64_data[i]!=0; ++i ){

        char encoded_character = base64_data[i];
        if (base64Validate(encoded_character) == -1) {
            fprintf(stderr, "InputError: the string to be decoded is not correctly encoded");
            exit(1);
        }
    /*
        checks for encoded data simultaneously, then subtracts
        and evaluated decimal to binary function, then copies it
        in Ox49_val_bin
    */  string Ox49_val_bin;
        if( encoded_character>='A' && encoded_character<='Z' )
            Ox49_val_bin = decimal_to_binary(encoded_character-65, 6);
        else if( encoded_character>='a' && encoded_character<='z' )
            Ox49_val_bin = decimal_to_binary(encoded_character-71, 6);
        else if( encoded_character>='0' && encoded_character<='9' )
            Ox49_val_bin = decimal_to_binary(encoded_character+4, 6);
        else if( encoded_character=='+')
            Ox49_val_bin = decimal_to_binary(encoded_character+19, 6);
        else
            Ox49_val_bin = decimal_to_binary(encoded_character+16, 6);

        strncat(bin_dump, Ox49_val_bin, 6);
        free(Ox49_val_bin);
    }

    int bin_dump_len = strlen(bin_dump);
    while( bin_dump_len%8 != 0 ){
    /*
        checks for 8 bit binary, if not it starts
        adding zeroes from the 0th index until it's
        8 bit binary value.
    */
        bin_dump[bin_dump_len++] = 0x30;
    }
    *(bin_dump+bin_dump_len) = '\0';

    i = 0, j = 0;
    while( bin_dump[i]!='\0' ){
    /*
        moves 1 byte from bin_dump to byte_bin,
        converts that byte binary to decimal and stores
        it in decodeData and increments i by 8.
    */
        memset(byte_bin, 0, strlen(byte_bin));
        memmove(byte_bin, bin_dump+i, 8);
        byte_bin[8] = 0;

        decodeData[j++] = binary_to_decimal(byte_bin);
        i += 8;
    }
    decodeData[j] = '\0';

    free(bin_dump);

    puts(decodeData);
    free(decodeData);

}


int main(void) {

    atexit(dalloc);

    string mode = input("Enter the mode(e/d): ");
    string data = input("Enter the data: ");
    if(!strcmp(data, "")){
        puts("DataError: no data provided");
        exit(1);
    }
    if(!strcmp(mode, "e"))
        encode(data);
    else if (!strcmp(mode, "d"))
        decode(data);
    else
        puts("ModeError: not a valid mode!");
    
    return 0;
}