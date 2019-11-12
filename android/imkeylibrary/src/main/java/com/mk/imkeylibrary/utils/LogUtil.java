package com.mk.imkeylibrary.utils;

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
        d("hhhhhhh：" + CTreble.INSTANCE.treble(2));
    }

    public interface CTreble extends Library {
        CTreble INSTANCE = (CTreble) Native.load("connector",CTreble.class);

        int treble(int value);
        int jni_call_static_method_safe();
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

    private static native void installApplet(Sender sender);
}
