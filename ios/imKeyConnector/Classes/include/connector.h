#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

const char *get_se_id(const char *(*callback)(const char *apdu));

char *rust_hello(const char *to, const char *(*callback)(const char *apdu));

void rust_hello_free(char *s);
