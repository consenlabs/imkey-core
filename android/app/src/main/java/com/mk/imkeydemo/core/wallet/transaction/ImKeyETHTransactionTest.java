package com.mk.imkeydemo.core.wallet.transaction;

import android.content.Context;

import org.json.JSONObject;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

import com.mk.imkeydemo.utils.ResourcesManager;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.wallet.Eth;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyBitcoinTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyEthereumTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.TransactionSignedResult;
import com.mk.imkeylibrary.exception.ImkeyException;

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

    }

    public static TransactionSignedResult testEthTxSign() {
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
    }

    public static String testEthMsgSign() {
        String data = "Hello imKey";
        String sender = "0x6031564e7b2F5cc33737807b2E58DaFF870B590b";
        return new Eth().signPersonalMessage(Path.ETH_LEDGER, data, sender);
    }

}
