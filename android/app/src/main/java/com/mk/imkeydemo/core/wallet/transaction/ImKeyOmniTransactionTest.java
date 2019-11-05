package com.mk.imkeydemo.core.wallet.transaction;

import android.content.Context;

import org.json.JSONArray;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

import com.mk.imkeydemo.utils.ResourcesManager;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyBitcoinTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyOmniTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.TransactionSignedResult;
import com.mk.imkeylibrary.exception.ImkeyException;

public class ImKeyOmniTransactionTest {

    //contains failCount、successCount、failed test case name
    public static Map<String, Object> result = new HashMap<String, Object>();

    public static Map<String, Object> testUxdtTxSign(Context context) {
        int failCount = 0;
        int successCount = 0;
        ArrayList<String> failedCaseName = new ArrayList<>();

        JSONObject testcases = ResourcesManager.getFromRaw(context, "usdttransactiontest");
        Iterator<String> keys = testcases.keys();
        try {
            while (keys.hasNext()) {
                ArrayList<ImKeyOmniTransaction.UTXO> utxo = new ArrayList<>();

                String key = keys.next();
                JSONObject testcase = testcases.getJSONObject(key);
                JSONArray utxoArray = testcase.getJSONArray("utxo");
                for (int i = 0; i < utxoArray.length(); i++) {

                    JSONObject utxoObj = utxoArray.getJSONObject(i);

                    utxo.add(new ImKeyOmniTransaction.UTXO(
                            utxoObj.getString("txHash"),
                            utxoObj.getInt("vout"),
                            utxoObj.getLong("amount"),
                            utxoObj.getString("address"),
                            utxoObj.getString("scriptPubKey"),
                            utxoObj.getString("derivedPath")
                    ));

                }

                ImKeyOmniTransaction transaction = new ImKeyOmniTransaction(
                        testcase.getString("to"),
                        testcase.getLong("amount"),
                        testcase.getLong("fee"),
                        testcase.getInt("propertyId"),
                        utxo,
                        testcase.getString("payment"),
                        testcase.getString("toDis"),
                        testcase.getString("from"),
                        testcase.getString("feeDis")
                        );

                Boolean retry = true;
                int tryCount = 0;
                while(retry) {
                    tryCount ++;
                    try {
                        TransactionSignedResult result = transaction.signTransaction(Constants.MAINNET, Path.BTC_PATH_PREFIX);
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

    }


    public static Map<String, Object> testUsdtSegwitTxSign(Context context) {
        int failCount = 0;
        int successCount = 0;
        ArrayList<String> failedCaseName = new ArrayList<>();

        JSONObject testcases = ResourcesManager.getFromRaw(context, "usdtsegwittransactiontest");
        Iterator<String> keys = testcases.keys();
        try {
            while (keys.hasNext()) {
                ArrayList<ImKeyBitcoinTransaction.UTXO> utxo = new ArrayList<>();

                String key = keys.next();
                JSONObject testcase = testcases.getJSONObject(key);
                JSONArray utxoArray = testcase.getJSONArray("utxo");
                for (int i = 0; i < utxoArray.length(); i++) {

                    JSONObject utxoObj = utxoArray.getJSONObject(i);

                    utxo.add(new ImKeyBitcoinTransaction.UTXO(
                            utxoObj.getString("txHash"),
                            utxoObj.getInt("vout"),
                            utxoObj.getLong("amount"),
                            utxoObj.getString("address"),
                            utxoObj.getString("scriptPubKey"),
                            utxoObj.getString("derivedPath")
                    ));

                }

                ImKeyOmniTransaction transaction = new ImKeyOmniTransaction(
                        testcase.getString("to"),
                        testcase.getLong("amount"),
                        testcase.getLong("fee"),
                        testcase.getInt("propertyId"),
                        utxo,
                        testcase.getString("payment"),
                        testcase.getString("toDis"),
                        testcase.getString("from"),
                        testcase.getString("feeDis")
                );

                Boolean retry = true;
                int tryCount = 0;
                while(retry) {
                    tryCount ++;
                    try {
                        TransactionSignedResult result = transaction.signSegWitTransaction(Constants.MAINNET, Path.BTC_SEGWIT_PATH_PREFIX);
                        String txHash = result.getTxHash();
                        String wtxID = result.getWtxID();
                        if(txHash.equals(testcase.getString("txHash")) && wtxID.equals(testcase.getString("wtxID"))) {
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

    public static TransactionSignedResult testUxdtTxSign() {
        ArrayList<ImKeyOmniTransaction.UTXO> utxo = new ArrayList<>();

        utxo.add(new ImKeyOmniTransaction.UTXO(
                "0dd195c815c5086c5995f43a0c67d28344ae5fa130739a5e03ef40fea54f2031", 0,
                14824854, "mkeNU5nVnozJiaACDELLCsVUc8Wxoh1rQN",
                "76a914383fb81cb0a3fc724b5e08cf8bbd404336d711f688ac",
                "0/0"));

        ImKeyOmniTransaction transaction = new ImKeyOmniTransaction("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3", 10050000000L,
                4000, 31, utxo, "100 USDT", "moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3",
                "2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB", "0.0004 BTC");

        TransactionSignedResult result = transaction.signTransaction(Constants.TESTNET, Path.BITCOIN_TESTNET_PATH);
        return result;
    }

    public static TransactionSignedResult testUsdtSegwitTxSign() {
        ArrayList<ImKeyOmniTransaction.UTXO> utxo = new ArrayList<>();

        utxo.add(new ImKeyOmniTransaction.UTXO(
                "9baf6fd0e560f9f199f4879c23cb73b9c4affb54a1cfdbacb85687efa89f4c78", 1,
                21863396, "2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB",
                "a9142d2b1ef5ee4cf6c3ebc8cf66a602783798f7875987",
                "0/0"));

        ImKeyOmniTransaction transaction = new ImKeyOmniTransaction("moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3", 10000000000L,
                4000, 31, utxo, "100 USDT", "moLK3tBG86ifpDDTqAQzs4a9cUoNjVLRE3",
                "2MwN441dq8qudMvtM5eLVwC3u4zfKuGSQAB", "0.0004 BTC");
        TransactionSignedResult result = transaction.signSegWitTransaction(Constants.TESTNET, Path.BITCOIN_SEGWIT_TESTNET_PATH);

        return result;
    }

}
