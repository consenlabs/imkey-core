#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * dispatch protobuf rpc call
 * //@@XM TODO: add in error handling
 */
const char *call_tcx_api(const char *hex_str);

void clear_err(void);

void get_address(void);

const char *get_apdu(void);

const char *get_apdu_return(void);

const char *get_last_err_message(void);

const char *get_seid(void);

void set_apdu(const char *apdu);

void set_apdu_return(const char *apdu_return);

void sign_transaction(void);
