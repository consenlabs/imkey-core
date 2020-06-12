package com.mk.imkeylibrary.keycore;

import android.os.SystemClock;
import android.text.TextUtils;

import com.google.protobuf.InvalidProtocolBufferException;
import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;
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


    public static byte[] callApi(String paramHex) {
        RustApi.INSTANCE.clear_err();
        String res = RustApi.INSTANCE.call_imkey_api(paramHex);
        String error = RustApi.INSTANCE.get_last_err_message();
        if(!TextUtils.isEmpty(error)){
            api.Api.Response response = null;
            try {
                response = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
            } catch (InvalidProtocolBufferException e) {
                e.printStackTrace();
                throw new ImkeyException(e.getMessage());
            }
            if(!response.getIsSuccess()){
                throw new ImkeyException(response.getError());
            }
        }

        return NumericUtil.hexToBytes(res);
    }
}
