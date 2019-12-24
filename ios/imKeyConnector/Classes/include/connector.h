#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void active_device(void);

void app_delete(void);

void app_download(void);

void app_update(void);

/**
 * dispatch protobuf rpc call
 * //@@XM TODO: add in error handling
 */
const char *call_tcx_api(const char *hex_str);

void check_device(void);

void check_update(void);

void get_address(void);

const char *get_apdu(void);

const char *get_apdu_return(void);

const char *get_se_id(const char *(*callback)(const char *apdu));

const char *get_seid(void);

char *rust_hello(const char *to, const char *(*callback)(const char *apdu));

void rust_hello_free(char *s);

void set_apdu(const char *apdu);

void set_apdu_return(const char *apdu_return);

void sign_transaction(void);
