package com.mk.imkeydemo.keycore;

import com.sun.jna.Library;
import com.sun.jna.Native;

public interface RustApi extends Library{

    RustApi INSTANCE = (RustApi) Native.load("connector",RustApi.class);

    String get_apdu();
    String set_apdu(String apdu);
    void set_apdu_return(String apdu_return);
    String get_apdu_return();
    void set_callback(Sender sender);
    void imkey_free_const_string(String str);

    String call_imkey_api(String value);
    void imkey_clear_err();
    String imkey_get_last_err_message();
}
