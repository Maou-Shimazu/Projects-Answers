#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<ctype.h>


int power(int base, int p){
   int result = 1;
   if(p){  
      for(int i = p; i > 0; i--){  
       
         result *= base;
      }
      return result;
   }
   else{  
      return result;  
   }
}


int purge(char*data, int index, int length){

    int i = index;
    for (; i < length; i++){
        *(data+i) = *(data+i+1);
    }
    *(data+i) = '\0';
    length--;
    return length;

}


void reverse(char*str){
   
   int i = 0, j = strlen(str)-1;
   while(i < j){
      int temp  = *(str+i);
      *(str+i++) = *(str+j);
      *(str+j--) = temp;
   }
}


int binary_to_decimal(char* binary){  
    
    int i = 0, j, k;
    int dec = 0;
    while(*(binary+i) != '\0')
        ++i;
    for(j = i-1, k = 0; j >= 0; --j, ++k){
        dec += (*(binary+k)-48)*power(2,j); 
    }
    return dec;

}


char* decimal_to_binary(int n, size_t padding){ 
    
    unsigned int dec = n;
    char binary[9]; 
    int i = 0;

    while(dec){ 

        int remain = dec % 2; 
        binary[i++] = remain+48; 
        dec = dec/2; 
    }   
    while(i != padding) {
        binary[i++] = '0';
    }
    binary[i] = '\0';
    reverse(binary);
    return strdup(binary);
    
}   


int charValidate(char ch) {

    if(ch < 0 || ch > 126)
        return -1;
    return 0;
}


int base64Validate(char b64ec) {
    
    if(b64ec < 43 || b64ec > 126 || (b64ec >= 44 && b64ec <47))
        return -1;
    if((b64ec > 57 && b64ec < 65) || (b64ec > 90 && b64ec < 97))
        return -1;
    if(b64ec > 122 && b64ec < 126)
        return -1;
    return 0;
}


void delspace(char* s) {
    char* d = s;
    do {
        while (*d == ' ') {
            ++d;
        }
    } while ((*s++ = *d++));
}