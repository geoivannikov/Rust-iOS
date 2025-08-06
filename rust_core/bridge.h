#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *encrypt_password(const char *ptr);

char *decrypt_password(const char *ptr);

void init_store(const char *path, const uint8_t *key_ptr);

void save_password(const char *tag, const char *password);

char *get_password(const char *tag);

bool tag_exists(const char *tag);

void free_string(char *ptr);

char *generate_password(uint32_t len);
