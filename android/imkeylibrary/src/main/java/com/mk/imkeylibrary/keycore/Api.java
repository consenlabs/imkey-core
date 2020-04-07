package com.mk.imkeylibrary.keycore;

import android.os.SystemClock;
import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.Sender;

public class Api {

    public static void startMessageDeamon(){
        new Thread(new Runnable() {
            @Override
            public void run() {
                while (true){
                    LogUtil.d("start while...");
                    String apdu = "";
                    while (true){
                        apdu = RustApi.INSTANCE.get_apdu();
                        if (!apdu.equals("")){
                            RustApi.INSTANCE.set_apdu("");
                            break;
                        }
                        SystemClock.sleep(1000);
                    }

                    String res = Ble.getInstance().sendApdu(apdu);

                    String apduRet = "";
                    while (true){
                        apduRet = RustApi.INSTANCE.get_apdu_return();
                        if(apduRet.equals("")){
                            RustApi.INSTANCE.set_apdu_return(res);
                            break;
                        }
                        SystemClock.sleep(1000);
                    }

                }
            }
        }).start();
    }

    public static void setCallback(){
        RustApi.INSTANCE.set_callback(new Sender() {
            @Override
            public String sendApdu(String apdu, int timeout) {
                LogUtil.d("set call back sucess");
                RustApi.INSTANCE.free_const_string(apdu);
                return Ble.getInstance().sendApdu(apdu,timeout);
            }
        });
    }

}
