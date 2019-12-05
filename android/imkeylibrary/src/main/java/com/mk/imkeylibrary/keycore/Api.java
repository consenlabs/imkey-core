package com.mk.imkeylibrary.keycore;

import android.os.SystemClock;
import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.utils.LogUtil;

public class Api {
    public static String getSeid(){
        return RustApi.INSTANCE.get_seid();
    }

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

    /**
     * for debug
     */
    public static void initRustLog(){
        RustApi.INSTANCE.init();
    }
}
