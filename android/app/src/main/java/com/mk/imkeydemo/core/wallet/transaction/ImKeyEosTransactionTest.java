package com.mk.imkeydemo.core.wallet.transaction;

import android.content.Context;

import org.json.JSONArray;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

import com.mk.imkeydemo.utils.ResourcesManager;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.wallet.Eos;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyBitcoinTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyEOSTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.TxMultiSignResult;
import com.mk.imkeylibrary.exception.ImkeyException;

public class ImKeyEosTransactionTest {

    //contains failCount、successCount、failed test case name
    public static Map<String, Object> result = new HashMap<String, Object>();

    public static Map<String, Object> testEosTxSign(Context context) {
        int failCount = 0;
        int successCount = 0;
        ArrayList<String> failedCaseName = new ArrayList<>();

        JSONObject testcases = ResourcesManager.getFromRaw(context, "eostransactiontest");
        Iterator<String> keys = testcases.keys();
        try {
            while (keys.hasNext()) {
                ArrayList<ImKeyBitcoinTransaction.UTXO> utxo = new ArrayList<>();

                String key = keys.next();
                JSONObject testcase = testcases.getJSONObject(key);

                List<String> publicKeyList = new ArrayList<String>();
                JSONArray publicKeys = testcase.getJSONArray("publicKeys");
                for (int i = 0; i < publicKeys.length(); i++) {
                    JSONObject publicKeyOjb = publicKeys.getJSONObject(i);
                    publicKeyList.add(publicKeyOjb.getString("publicKey"));
                }
                ImKeyEOSTransaction.ToSignObj toSignObj = new ImKeyEOSTransaction.ToSignObj();
                toSignObj.setPublicKeys(publicKeyList);
                toSignObj.setTxHex(testcase.getString("txHex"));

                List<ImKeyEOSTransaction.ToSignObj> toSignObjs = new ArrayList<>();
                toSignObjs.add(toSignObj);
                ImKeyEOSTransaction eosTransaction = new ImKeyEOSTransaction(toSignObjs);

                String chainId = testcase.getString("chainId");
                JSONObject pre = testcase.getJSONObject("preview");
                String paymentDis = pre.getString("payment");
                String receiverDis = pre.getString("receiver");
                String senderDis = pre.getString("sender");

                Boolean retry = true;
                int tryCount = 0;
                while(retry) {
                    tryCount ++;
                    try {
                        List<TxMultiSignResult> signResults = eosTransaction.signTransactions(chainId, receiverDis, senderDis, paymentDis, Path.EOS_LEDGER);
                        String txHash = signResults.get(0).getTxHash();
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

    public static List<TxMultiSignResult> testEosTxSign() {
        List<ImKeyEOSTransaction.ToSignObj> toSignObjs = new ArrayList<>();
        ImKeyEOSTransaction.ToSignObj toSignObj = new ImKeyEOSTransaction.ToSignObj();
        toSignObj.setPublicKeys(Collections.singletonList("EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF"));
        toSignObj.setTxHex("c578065b93aec6a7c811000000000100a6823403ea3055000000572d3ccdcd01000000602a48b37400000000a8ed323225000000602a48b374208410425c95b1ca80969800000000000453595300000000046d656d6f00");
        toSignObjs.add(toSignObj);
        ImKeyEOSTransaction eosTransaction = new ImKeyEOSTransaction(toSignObjs);
        String chainIdEos = "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906";
        String to = "bbbb5555bbbb";
        String from = "liujianmin12";
        String payment = "undelegatebw 0.0100 EOS";
        List<TxMultiSignResult> signResults = eosTransaction.signTransactions(chainIdEos, to, from, payment, Path.EOS_LEDGER);
        return signResults;
    }

    public static String testEosMsgSign() {
        String PUBLIC_KEY = "EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF";
        return new Eos().eosEcSign("imKey2019", false, PUBLIC_KEY,Path.EOS_LEDGER);
    }
}
