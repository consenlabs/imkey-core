package com.mk.imkeydemo.core.wallet.transaction;

import com.google.protobuf.Any;
import com.mk.imkeydemo.keycore.RustApi;
import com.mk.imkeydemo.keycore.TransactionSignedResult;
import com.mk.imkeydemo.utils.NumericUtil;

import org.json.JSONObject;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

import api.Api;
import common.Common;
import filecoinapi.Filecoin;
import im.imkey.imkeylibrary.utils.ByteUtil;
import im.imkey.imkeylibrary.utils.LogUtil;

public class ImKeyFilecoinTransactionTest {

    //contains failCount、successCount、failed test case name
    public static Map<String, Object> result = new HashMap<String, Object>();


    public static TransactionSignedResult testFilecoinTxSign() {

        TransactionSignedResult signResults = null;

        try {

            Filecoin.FilecoinTxInput txReq = Filecoin.FilecoinTxInput.newBuilder()
                    .setTo("f1d2xrzcslx7xlbbylc5c3d5lvandqw4iwl6epxba")
                    .setFrom("f1o2ph66tg7o7obyrqa7eiwiinrltauzxitkuk4ay")
                    .setNonce(1)
                    .setValue("100000")
                    .setGasLimit(1)
                    .setGasFeeCap("1")
                    .setGasPremium("1")
                    .setMethod(0)
                    .setParams("")
                    .build();
            Any inputAny = Any.newBuilder()
                    .setValue(txReq.toByteString())
                    .build();
//            filecoinapi.Filecoin.FilecoinTxReq txReq = filecoinapi.Filecoin.FilecoinTxReq.newBuilder()
            Common.SignParam signParamBuild = Common.SignParam.newBuilder()
                    .setInput(inputAny)
                    .setChainType("FILECOIN")
                    .setPath("m/44'/461'/0/0/0")
                    .setNetwork("MAINNET")
                    .setPayment("1 FILECOIN")
                    .setReceiver("f1d2xrzcslx7xlbbylc5c3d5lvandqw4iwl6epxba")
                    .setSender("f1o2ph66tg7o7obyrqa7eiwiinrltauzxitkuk4ay")
                    .setFee("0.1 FILECOIN")
                    .build();

            Any signParamAny = Any.newBuilder()
                    .setValue(signParamBuild.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("sign_tx")
                    .setParam(signParamAny)
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
                Filecoin.FilecoinTxOutput response = Filecoin.FilecoinTxOutput.parseFrom(ByteUtil.hexStringToByteArray(result));
                String cid = response.getCid();
                int type = response.getSignature().getType();
                String signature = response.getSignature().getData();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("cid：" + cid);
                LogUtil.d("type：" + type);
                LogUtil.d("signature：" + signature);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");

                signResults = new TransactionSignedResult(signature, cid);
            }
        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }
        return signResults;
    }
}
