#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

void reverse(char*str){
   
   int i = 0, j = strlen(str)-1;
   while(i < j){
      int temp  = *(str+i);
      *(str+i++) = *(str+j);
      *(str+j--) = temp;
   }
}

char* binary(float n){ 
    
    int dec = n, initsize = 2;
    float after = n-(float)dec;
    int i = 0, flag = 0;

    char *binary = calloc(1, (size_t)ceil(dec/32)+10);
    if(n <= -0.0) {
        binary[i++] = '-';
        dec *= -1;
        flag = 1;
    } 
    while(dec){ 
        binary[i++] = (dec % 2)+48; 
        dec = dec/2; 
    }   
    while((i-flag)%4 != 0 || (i-flag) == 0) {
        binary[i++] = '0';
    }
    reverse(binary+flag);

    if (after != (float)0) {
        
        binary[i++] = '.';

        while(after != (int)after) {
            after *= 2;
            if((int)after)
                binary[i++] = '1';
            else 
                binary[i++] = '0';
        }
    }
    
    binary[i] = '\0';
    return binary;
    
}

int main(void) {

    float float_value;
    printf("Enter the float value: ");
    scanf("%f", &float_value);
    char* bin_value = binary(float_value);
    puts(bin_value);
    free(bin_value);
}