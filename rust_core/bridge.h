#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *encrypt_password(const char *ptr);

char *decrypt_password(const char *ptr);

void free_string(char *ptr);

char *generate_password(uint32_t len);
