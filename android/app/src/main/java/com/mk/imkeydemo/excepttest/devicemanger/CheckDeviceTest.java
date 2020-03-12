package com.mk.imkeydemo.excepttest.devicemanger;

import com.google.protobuf.Any;
import com.mk.imkeylibrary.keycore.RustApi;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class CheckDeviceTest {

    public void checkDeviceTest() {

        String error = null;

        //imkey_tsm_device_stop_using
        RustApi.INSTANCE.clear_err();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("se_secure_check")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            error = RustApi.INSTANCE.get_last_err_message();
            api.Api.Response response = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
            response.getIsSuccess();
            error = response.getError();

            LogUtil.e("期望结果：imkey_tsm_device_stop_using" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }


    }
}
