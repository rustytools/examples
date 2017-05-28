#include <stdio.h>
#include <string.h>

char* hello(const char* name, int name_len, char** out, int* out_len);
void hello_free(char* buf);

int main() {
    const char* msg = "good old C";
    char* out = NULL;
    int out_len = 0;
    char* err = hello(msg, strlen(msg), &out, &out_len);
    if (err) {
        printf("error: reported: %s\n", err);
        hello_free(err);
        return 1;
    }
    if (!out) {
        printf("error: null out: %s\n", err);
        return 1;
    }
    if (strlen(out) != out_len) {
        printf("error: wrong out_len: %s\n", err);
        return 1;
    }
    puts(out);
    hello_free(out);

    return 0;
}
