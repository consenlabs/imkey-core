package com.mk.imkeydemo.keycore;

import com.google.protobuf.Any;
import com.mk.imkeydemo.utils.NumericUtil;

import api.Api;
import im.imkey.imkeylibrary.utils.ByteUtil;
import im.imkey.imkeylibrary.utils.LogUtil;

public class Filecoin extends Wallet {

    public String getAddress(String path) {
        String address = null;

        try {
            Api.AddressParam req = Api.AddressParam.newBuilder()
                    .setPath(path)
                    .setNetwork("MAINNET")
                    .setChainType("FILECOIN")
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("get_address")
                    .setParam(any)
                    .build();

            String hex = NumericUtil.bytesToHex(action.toByteArray());

            // clear_err
            RustApi.INSTANCE.imkey_clear_err();

            String result = RustApi.INSTANCE.call_imkey_api(hex);

            String error = RustApi.INSTANCE.imkey_get_last_err_message();
            if(!"".equals(error) && null != error) {
                Api.ErrorResponse errorResponse = Api.ErrorResponse.parseFrom(ByteUtil.hexStringToByteArray(error));
                Boolean isSuccess = errorResponse.getIsSuccess();
                if(!isSuccess) {
                    LogUtil.d("异常： " + errorResponse.getError());

                }
            } else {
                Api.AddressResult response = Api.AddressResult.parseFrom(ByteUtil.hexStringToByteArray(result));
                address = response.getAddress();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("address：" + address);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
            }

        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }

        return address;

    }

    public String displayAddress(String path) {

        String address = null;

        try {
            Api.AddressParam req = Api.AddressParam.newBuilder()
                    .setPath(path)
                    .setNetwork("MAINNET")
                    .setChainType("FILECOIN")
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("register_address")
                    .setParam(any)
                    .build();

            String hex = NumericUtil.bytesToHex(action.toByteArray());

            // clear_err
            RustApi.INSTANCE.imkey_clear_err();

            String result = RustApi.INSTANCE.call_imkey_api(hex);

            String error = RustApi.INSTANCE.imkey_get_last_err_message();
            if(!"".equals(error) && null != error) {
                Api.ErrorResponse errorResponse = Api.ErrorResponse.parseFrom(ByteUtil.hexStringToByteArray(error));
                Boolean isSuccess = errorResponse.getIsSuccess();
                if(!isSuccess) {
                    LogUtil.d("异常： " + errorResponse.getError());

                }
            } else {
                Api.AddressResult response = Api.AddressResult.parseFrom(ByteUtil.hexStringToByteArray(result));
                address = response.getAddress();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("address：" + address);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
            }

        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }

        return address;
    }
}
