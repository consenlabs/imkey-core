package com.mk.imkeydemo.keycore;

import com.google.protobuf.Any;
import com.mk.imkeydemo.utils.NumericUtil;

import deviceapi.Device;
import im.imkey.imkeylibrary.utils.ByteUtil;
import im.imkey.imkeylibrary.utils.LogUtil;

public class DeviceBidingExample {
    public static void bindCheck() {

        String error = null;
        String bindCode = "123456789";

//        deviceapi.Device. req = deviceapi.Device.BindCheckReq.newBuilder()
//                .setFilePath("")
//                .build();
//
//        Any any = Any.newBuilder()
//                .setValue(req.toByteString())
//                .build();

        api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                .setMethod("bind_check")
//                .setParam(any)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());
        try {
            String result = RustApi.INSTANCE.call_imkey_api(hex);
            error = RustApi.INSTANCE.imkey_get_last_err_message();
            Device.BindCheckRes response = Device.BindCheckRes.parseFrom(ByteUtil.hexStringToByteArray(error));

//            response.getIsSuccess();
//            error = response.getError();

            LogUtil.e("期望结果：imkey_sdk_illegal_argument" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }

    }

    public static void bindAcquire() {

        String error = null;
        String bindCode = "YDSGQPKX";

        deviceapi.Device.BindAcquireReq req = deviceapi.Device.BindAcquireReq.newBuilder()
                .setBindCode(bindCode)
                .build();

        Any any = Any.newBuilder()
                .setValue(req.toByteString())
                .build();

        api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                .setMethod("bind_acquire")
                .setParam(any)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());
        try {
            String result = RustApi.INSTANCE.call_imkey_api(hex);
            error = RustApi.INSTANCE.imkey_get_last_err_message();
            Device.BindCheckRes response = Device.BindCheckRes.parseFrom(ByteUtil.hexStringToByteArray(error));
//            response.getIsSuccess();
//            error = response.getError();

            LogUtil.e("期望结果：imkey_sdk_illegal_argument" + "，实际结果：" + error);

        } catch (Exception e) {
            e.printStackTrace();
        }

    }
}
