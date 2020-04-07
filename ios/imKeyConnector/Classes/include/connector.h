#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * dispatch protobuf rpc call
 */
const char *call_imkey_api(const char *hex_str);

void clear_err(void);

const char *get_apdu(void);

const char *get_apdu_return(void);

const char *get_last_err_message(void);

void set_apdu(const char *apdu);

void set_apdu_return(const char *apdu_return);

void set_callback(const char *(*callback)(const char *apdu, int32_t timeout));
