#ifndef UTILS_H
#define UTILS_H
//Function declarations

void reverse(char*str);
int power(int base, int exp);
int purge(char*data, int index, int length);
char* decimal_to_binary(int dec, int padding);
int binary_to_decimal(char* bin);
int charValidate(char ch);
int base64Validate(char b64ed);
void delspace(char* s);

#endif