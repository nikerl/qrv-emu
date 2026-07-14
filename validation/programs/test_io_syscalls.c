#include <stdio.h>

int main(void) {
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("test.txt", "r");

    // Store the content of the file
    char str1[100];

    // Read the content and store it inside str1
    fgets(str1, 100, fptr);

    // Print the file content
    printf("%s", str1);

    // Close the file
    fclose(fptr); 

    // Store the content of the file
    char str2[100];

    // Read the content and store it inside str2
    fgets(str2, 100, stdin);

    // Print the file content
    printf("%s", str2);

    return 0;
}
