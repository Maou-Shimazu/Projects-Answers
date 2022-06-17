#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdint.h>
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

char* IEEE_754_32bit(float value) {

    char mantissa[24], _32_bit[33] ={'0'};

    char* decbin = binary(value);
    if(*decbin == '-'){
        decbin = decbin+1;
        _32_bit[0] = '1';
    } else if(*decbin == '+')
        decbin = decbin+1;

    float floatbin = atof(decbin);
    int exp = 0;
    while((int)floatbin != 1){
        if((int)floatbin > 1) {
            floatbin /= 10;
            exp++;
        } else if(!((int)floatbin)) {
            floatbin *= 10;
            exp--;
        }
    }
    int i = 0;
    if(exp > 0) {
        for(; i < exp; i++){
            mantissa[i] = decbin[1+i];
        }
        mantissa[i++] = 0;

        int idx = (int)strcspn(decbin, ".")+1;
        strcat(mantissa, decbin+idx);
        memset(mantissa+strlen(mantissa), '0', 23);
    } else 
        memset(mantissa, '0', 23);
    mantissa[23] = 0;
    free(decbin);

    int bias = exp + 127;
    char* bin = binary(bias);
    strcat(_32_bit, bin);
    strcat(_32_bit, mantissa);
    free(bin);

    return strdup(_32_bit);
        
}

int main(void) {

    float value;
    printf("Enter the float value: ");
    scanf("%f", &value);
    char* bin32 = IEEE_754_32bit(value);
    puts(bin32);
    free(bin32);

}

