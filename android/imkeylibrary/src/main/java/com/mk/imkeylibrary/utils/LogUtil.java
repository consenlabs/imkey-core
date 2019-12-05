package com.mk.imkeylibrary.utils;

import android.os.SystemClock;
import android.util.Log;

import com.mk.imkeylibrary.BuildConfig;
import com.mk.imkeylibrary.bluetooth.Ble;
import com.sun.jna.Library;
import com.sun.jna.Native;

public class LogUtil {
    private static final boolean DEBUG = BuildConfig.DEBUG;
    private static final String TAG = "imkey";

    public static void d(String msg) {
        if (DEBUG)
            Log.d(TAG, msg);
    }

    public static void d(String tag, String msg) {
        if (DEBUG)
            Log.d(tag, msg);
    }

    public static void e(String msg){
        if(DEBUG)
            Log.e(TAG, msg);
    }

    public static void e(String tag, String msg) {
        if(DEBUG)
            Log.e(tag, msg);
    }

    public static void hh(){
//        d("hhhhhhh：" + CTreble.INSTANCE.get_seid());
//        CTreble.INSTANCE.init();
    }

    public static String getSeid(){
        new Thread(new Runnable() {
            @Override
            public void run() {
                while (true){
                    LogUtil.d("start while...");
                    String apdu = "";
                    while (true){
                        apdu = CTreble.INSTANCE.get_apdu();
                        if (!apdu.equals("")){
                            CTreble.INSTANCE.set_apdu("");
                            break;
                        }
                        SystemClock.sleep(1000);
                    }

                    String res = Ble.getInstance().sendApdu(apdu);

                    String apduRet = "";
                    while (true){
                        apduRet = CTreble.INSTANCE.get_apdu_return();
                        if(apduRet.equals("")){
                            CTreble.INSTANCE.set_apdu_return(res);
                            break;
                        }
                        SystemClock.sleep(1000);
                    }

                }
            }
        }).start();
        return CTreble.INSTANCE.get_seid();
    }

    public interface CTreble extends Library {
        CTreble INSTANCE = (CTreble) Native.load("connector",CTreble.class);

        int treble(int value);
        int jni_call_static_method_safe();

        String get_seid();
        String get_apdu();
        String set_apdu(String apdu);
        void set_apdu_return(String apdu_return);
        String get_apdu_return();

        String get_se_id(Sender sender);
        void init();
    }

    static {
        System.loadLibrary("connector");
    }

    public static void installApplet(){
        installApplet(new Sender() {
            @Override
            public String sendApdu(String apdu) {
                LogUtil.d("test apdu ...." + apdu);
                return Ble.getInstance().sendApdu(apdu);
            }
        });
    }

    public static String getseidwithcallbacktest(){
        return CTreble.INSTANCE.get_se_id(new Sender() {
            @Override
            public String sendApdu(String apdu) {
                LogUtil.d("sender.. apdu ...." + apdu);
                return Ble.getInstance().sendApdu(apdu);
            }
        });
    }

    private static native void installApplet(Sender sender);
}
