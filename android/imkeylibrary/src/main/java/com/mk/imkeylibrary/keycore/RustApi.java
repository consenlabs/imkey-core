package com.mk.imkeylibrary.keycore;

import com.mk.imkeylibrary.utils.Sender;
import com.sun.jna.Library;
import com.sun.jna.Native;

public interface RustApi extends Library{
    RustApi INSTANCE = (RustApi) Native.load("connector",RustApi.class);

    String get_seid();
    String get_apdu();
    String set_apdu(String apdu);
    void set_apdu_return(String apdu_return);
    String get_apdu_return();

    String get_se_id(Sender sender);
    void init();

    void check_update();
}
