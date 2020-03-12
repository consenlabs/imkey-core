package com.mk.imkeydemo.excepttest.devicemanger;

import com.google.protobuf.Any;
import com.mk.imkeylibrary.keycore.RustApi;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;


public class AppDeleteTest {

    public void appDeleteTest() {

        String appName = "BTC";
        String appName2 = "TEST";
        String error = null;

        //imkey_illegal_prarm
        RustApi.INSTANCE.clear_err();

        deviceapi.Device.AppAction appAction = deviceapi.Device.AppAction.newBuilder()
                .setAppName(appName)
                .build();

        Any any = Any.newBuilder()
                .setValue(appAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("app_delete")
                //.setParam(any)
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

            LogUtil.e("期望结果：imkey_illegal_prarm" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }


        RustApi.INSTANCE.clear_err();

        appAction = deviceapi.Device.AppAction.newBuilder()
                .setAppName(appName2)
                .build();

        any = Any.newBuilder()
                .setValue(appAction.toByteString())
                .build();

        deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("app_delete")
                .setParam(any)
                .build();

        any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        hex = NumericUtil.bytesToHex(action.toByteArray());
        try {
            RustApi.INSTANCE.call_tcx_api(hex);
            error = RustApi.INSTANCE.get_last_err_message();
            api.Api.Response response = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
            response.getIsSuccess();
            error = response.getError();

            LogUtil.e("期望结果：imkey_app_name_not_exist" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }


        //设备未激活，需要改后台数据测试
        RustApi.INSTANCE.clear_err();

        appAction = deviceapi.Device.AppAction.newBuilder()
                .setAppName(appName)
                .build();

        any = Any.newBuilder()
                .setValue(appAction.toByteString())
                .build();

        deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("app_delete")
                .setParam(any)
                .build();

        any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        hex = NumericUtil.bytesToHex(action.toByteArray());
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            error = RustApi.INSTANCE.get_last_err_message();
            api.Api.Response response = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
            response.getIsSuccess();
            error = response.getError();

            LogUtil.e("期望结果：imkey_tsm_device_not_activated" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }

    }

}
