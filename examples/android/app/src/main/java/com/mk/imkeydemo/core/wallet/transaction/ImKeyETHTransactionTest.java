package com.mk.imkeydemo.core.wallet.transaction;

import android.content.Context;

import com.google.protobuf.Any;
import com.mk.imkeydemo.keycore.Path;
import com.mk.imkeydemo.keycore.RustApi;
import com.mk.imkeydemo.keycore.TransactionSignedResult;
import com.mk.imkeydemo.utils.NumericUtil;
import com.mk.imkeydemo.utils.ResourcesManager;


import org.json.JSONObject;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

import api.Api;
import common.Common;
import ethapi.Eth;
import im.imkey.imkeylibrary.common.Messages;
import im.imkey.imkeylibrary.exception.ImkeyException;
import im.imkey.imkeylibrary.utils.ByteUtil;
import im.imkey.imkeylibrary.utils.LogUtil;

public class ImKeyETHTransactionTest {

    //contains failCount、successCount、failed test case name
    public static Map<String, Object> result = new HashMap<String, Object>();

    public static Map<String, Object> testEthTxSign(Context context) {
        int failCount = 0;
        int successCount = 0;
        ArrayList<String> failedCaseName = new ArrayList<>();

        JSONObject testcases = ResourcesManager.getFromRaw(context, "ethtransactiontest");
        Iterator<String> keys = testcases.keys();
        try {
            while (keys.hasNext()) {

                String key = keys.next();
                JSONObject testcase = testcases.getJSONObject(key);
                JSONObject tran = testcase.getJSONObject("transaction");

                BigInteger nonce = BigInteger.valueOf(tran.getLong("nonce"));
                BigInteger gasPrice = BigInteger.valueOf(tran.getLong("gasPrice"));
                BigInteger gasLimit = BigInteger.valueOf(tran.getLong("gasLimit"));
                String to = tran.getString("to");
                BigInteger value = BigInteger.valueOf(tran.getLong("value"));
                String data = tran.getString("data");
                String v = tran.getString("v");

                JSONObject pre = testcase.getJSONObject("preview");

                Eth.EthTxInput ethTxReq = ethapi.Eth.EthTxInput.newBuilder()
//                        .setPath(Path.ETH_LEDGER)
                        .setChainId(v)
                        .setNonce(nonce.toString())
                        .setGasPrice(gasPrice.toString())
                        .setGasLimit(gasLimit.toString())
                        .setTo(to)
                        .setValue(value.toString())
                        .setData(data)
//                        .setPayment(pre.getString("payment"))
//                        .setReceiver(pre.getString("receiver"))
//                        .setSender(pre.getString("sender"))
//                        .setFee(pre.getString("fee"))
                        .build();

                Any any = Any.newBuilder()
                        .setValue(ethTxReq.toByteString())
                        .build();

                api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                        .setMethod("eth_tx_sign")
                        .setParam(any)
                        .build();

                Boolean retry = true;
                int tryCount = 0;
                while(retry) {
                    tryCount ++;
                    try {

                        LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                        // clear_err
                        RustApi.INSTANCE.imkey_clear_err();

                        String hex = NumericUtil.bytesToHex(action.toByteArray());

                        String result = RustApi.INSTANCE.call_imkey_api(hex);

                        //
                        String error = RustApi.INSTANCE.imkey_get_last_err_message();
                        if(!"".equals(error) && null != error) {
                            Api.ErrorResponse errorResponse = Api.ErrorResponse.parseFrom(ByteUtil.hexStringToByteArray(error));
                            Boolean isSuccess = errorResponse.getIsSuccess();
                            if(!isSuccess) {
                                LogUtil.d("异常： " + errorResponse.getError());
                                failedCaseName.add(key);
                                failCount++;
                                retry = false;
                                continue;
                            }
                        }

                        Eth.EthTxOutput response = Eth.EthTxOutput.parseFrom(ByteUtil.hexStringToByteArray(result));
                        String txHash = response.getTxHash();
                        String signature = response.getSignature();
                        LogUtil.d("signature：" + signature);
                        LogUtil.d("txHash：" + txHash);
                        LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                        if(txHash.equals(testcase.getString("txHash"))) {
                            LogUtil.e("×××××××××××××××××××××××××××××××××××成功×××××××××××××××××××××××××××××××××××××××××××××××");
                            successCount ++;
                        } else {
                            failedCaseName.add(key);
                            failCount++;
                        }
                        retry = false;
                    } catch (ImkeyException e) {
                        if(!Messages.IMKEY_BLUETOOTH_CHANNEL_ERROR.equals(e.getMessage()) || tryCount >= 3) {
                            retry = false;
                            failedCaseName.add(key + ": " + e.getMessage());
                            failCount++;
                        }
                    }
                }
            }

        } catch (Exception e) {
            throw new ImkeyException(e);
        }

        result.put("failCount", failCount);
        result.put("successCount", successCount);
        result.put("failedCaseName", failedCaseName);

        return result;

    }


    /*public static Map<String, Object> testEthTxSign(Context context) {
        int failCount = 0;
        int successCount = 0;
        ArrayList<String> failedCaseName = new ArrayList<>();

        JSONObject testcases = ResourcesManager.getFromRaw(context, "ethtransactiontest");
        Iterator<String> keys = testcases.keys();
        try {
            while (keys.hasNext()) {
                ArrayList<ImKeyBitcoinTransaction.UTXO> utxo = new ArrayList<>();

                String key = keys.next();
                JSONObject testcase = testcases.getJSONObject(key);

                JSONObject tran = testcase.getJSONObject("transaction");

                BigInteger nonce = BigInteger.valueOf(tran.getLong("nonce"));
                BigInteger gasPrice = BigInteger.valueOf(tran.getLong("gasPrice"));
                BigInteger gasLimit = BigInteger.valueOf(tran.getLong("gasLimit"));
                String to = tran.getString("to");
                BigInteger value = BigInteger.valueOf(tran.getLong("value"));
                String data = tran.getString("data");
                String v = tran.getString("v");

                JSONObject pre = testcase.getJSONObject("preview");
                HashMap<String, String> preview = new HashMap<>();
                preview.put("payment", pre.getString("payment"));
                preview.put("receiver", pre.getString("receiver"));
                preview.put("sender", pre.getString("sender"));
                preview.put("fee", pre.getString("fee"));

                ImKeyEthereumTransaction ethTx = new ImKeyEthereumTransaction(nonce, gasPrice,gasLimit,to,value,data, preview);

                Boolean retry = true;
                int tryCount = 0;
                while(retry) {
                    tryCount ++;
                    try {
                        TransactionSignedResult result = ethTx.signTransaction(v, Path.ETH_LEDGER);
                        String txHash = result.getTxHash();
                        if(txHash.equals(testcase.getString("txHash"))) {
                            successCount ++;
                        } else {
                            failedCaseName.add(key);
                            failCount++;
                        }
                        retry = false;
                    } catch (ImkeyException e) {
                        if(!Messages.IMKEY_BLUETOOTH_CHANNEL_ERROR.equals(e.getMessage()) || tryCount >= 3) {
                            retry = false;
                            failedCaseName.add(key + ": " + e.getMessage());
                            failCount++;
                        }
                    }
                }
            }
        } catch (Exception e) {
            throw new ImkeyException(e);
        }

        result.put("failCount", failCount);
        result.put("successCount", successCount);
        result.put("failedCaseName", failedCaseName);

        return result;

    }*/


    public static TransactionSignedResult testEthTxSign() {

        TransactionSignedResult signResults = null;

        try {

            Eth.EthTxInput ethTxReq = ethapi.Eth.EthTxInput.newBuilder()
                    .setNonce("8")
                    .setGasPrice("20000000008")
                    .setGasLimit("189000")
                    .setTo("0x3535353535353535353535353535353535353535")
                    .setValue("512")
                    .setChainId("28")
                    .setType("00")
                    .build();

            Any inputAny = Any.newBuilder()
                    .setValue(ethTxReq.toByteString())
                    .build();

            Common.SignParam signParamBuild = Common.SignParam.newBuilder()
                    .setChainType("ETHEREUM")
                    .setPath("m/44'/60'/0'/0/0")
                    .setPayment("0.01 ETH")
                    .setInput(inputAny)
                    .setReceiver("0xE6F4142dfFA574D1d9f18770BF73814df07931F3")
                    .setSender("0x6031564e7b2F5cc33737807b2E58DaFF870B590b")
                    .setFee("0.0032 ether")
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
                Eth.EthTxOutput response = ethapi.Eth.EthTxOutput.parseFrom(ByteUtil.hexStringToByteArray(result));
                String txHash = response.getTxHash();
                String signature = response.getSignature();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("signature：" + signature);
                LogUtil.d("txHash：" + txHash);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");

                signResults = new TransactionSignedResult(signature, txHash);
            }
        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }
        return signResults;
    }

    /*public static TransactionSignedResult testEthTxSign() {
        HashMap<String, String> preview = new HashMap<>();
        preview.put("payment", "0.01 ETH");
        preview.put("receiver", "0xE6F4142dfFA574D1d9f18770BF73814df07931F3");
        preview.put("sender", "0x6031564e7b2F5cc33737807b2E58DaFF870B590b");
        preview.put("fee", "0.0032 ether");

        ImKeyEthereumTransaction ethereumTransaction = new ImKeyEthereumTransaction(BigInteger.valueOf(8L), BigInteger.valueOf(20000000008L),
                BigInteger.valueOf(189000L), "0x3535353535353535353535353535353535353535",
                BigInteger.valueOf(512), "", preview);

        TransactionSignedResult result = ethereumTransaction.signTransaction("28", Path.ETH_LEDGER);
        return result;
    }*/

    public static String testEthMsgSign() {



        String signature = null;

        try {

            String data = "Hello imKey";
            Eth.EthMessageInput ethMessageSignReq = Eth.EthMessageInput.newBuilder()
                    .setIsPersonalSign(true)
                    .setMessage(data)
                    .build();

            Any inputAny = Any.newBuilder()
                    .setValue(ethMessageSignReq.toByteString())
                    .build();
            Common.SignParam signParamBuild = Common.SignParam.newBuilder()
                    .setChainType("ETHEREUM")
                    .setPath("m/44'/60'/0'/0/0")
                    .setInput(inputAny)
                    .setSender("0x6031564e7b2F5cc33737807b2E58DaFF870B590b")
                    .build();
            Any signParamAny = Any.newBuilder()
                    .setValue(signParamBuild.toByteString())
                    .build();
            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("sign_message")
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
                Eth.EthMessageOutput response = Eth.EthMessageOutput.parseFrom(ByteUtil.hexStringToByteArray(result));
                signature = response.getSignature();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("signature：" + signature);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
            }
        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }
        return signature;
    }

    /*public static String testEthMsgSign() {
        String data = "Hello imKey";
        String sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b";
        return new Eth().signPersonalMessage(Path.ETH_LEDGER, data, sender);
    }*/

}
