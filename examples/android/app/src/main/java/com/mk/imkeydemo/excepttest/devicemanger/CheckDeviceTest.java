package com.mk.imkeydemo.excepttest.devicemanger;


import com.mk.imkeydemo.keycore.RustApi;
import com.mk.imkeydemo.utils.NumericUtil;

import api.Api;
import im.imkey.imkeylibrary.utils.ByteUtil;
import im.imkey.imkeylibrary.utils.LogUtil;

public class CheckDeviceTest {

    public void checkDeviceTest() {

        String error = null;

        //imkey_tsm_device_stop_using
        RustApi.INSTANCE.imkey_clear_err();

        api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                .setMethod("device_secure_check")
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());
        try {
            String result = RustApi.INSTANCE.call_imkey_api(hex);
            error = RustApi.INSTANCE.imkey_get_last_err_message();
            Api.ErrorResponse response = Api.ErrorResponse.parseFrom(ByteUtil.hexStringToByteArray(error));
            response.getIsSuccess();
            error = response.getError();

            LogUtil.e("期望结果：imkey_tsm_device_stop_using" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }


    }
}
