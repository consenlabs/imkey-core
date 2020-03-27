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

import com.google.protobuf.Any;
import com.mk.imkeydemo.utils.ResourcesManager;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.transaction.ImKeyBitcoinTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.TransactionSignedResult;
import com.mk.imkeylibrary.core.wallet.transaction.TxMultiSignResult;
import com.mk.imkeylibrary.core.wallet.transaction.cosmos.Coin;
import com.mk.imkeylibrary.core.wallet.transaction.cosmos.ImKeyCosmosTransaction;
import com.mk.imkeylibrary.core.wallet.transaction.cosmos.StdFee;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.keycore.RustApi;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class ImKeyCosmosTransactionTest {

    //contains failCount、successCount、failed test case name
    public static Map<String, Object> result = new HashMap<String, Object>();

    public static Map<String, Object> testCosmosTxSign(Context context) {
        int failCount = 0;
        int successCount = 0;
        ArrayList<String> failedCaseName = new ArrayList<>();

        JSONObject testcases = ResourcesManager.getFromRaw(context, "cosmostransactiontest");
        Iterator<String> keys = testcases.keys();
        try {
            while (keys.hasNext()) {
                ArrayList<ImKeyBitcoinTransaction.UTXO> utxo = new ArrayList<>();

                String key = keys.next();
                JSONObject testcase = testcases.getJSONObject(key);
                JSONObject fee = testcase.getJSONObject("fee");

                List<Coin> amountList = new ArrayList<Coin>();
                JSONArray amount = fee.getJSONArray("amount");
                for (int i = 0; i < amount.length(); i++) {

                    JSONObject amountOjb = amount.getJSONObject(i);
                    Long feeAmount = amountOjb.getLong("amount");
                    String feeDenom = amountOjb.getString("denom");

                    Coin coin = new Coin(feeDenom, feeAmount);
                    amountList.add(coin);
                }
                Long gas = fee.getLong("gas");
                StdFee stdFee = new StdFee(amountList, gas);

                List<Map<String, Object>> msgList = new ArrayList<Map<String, Object>>();
                JSONArray msg = testcase.getJSONArray("msg");
                for (int i = 0; i < msg.length(); i++) {
                    JSONObject msgOjb = msg.getJSONObject(i);

                    Map<String, Object> msgMap = new HashMap<>();
                    String type = msgOjb.getString("type");
                    msgMap.put("type", type);

                    //设置value
                    JSONObject value = msgOjb.getJSONObject("value");
                    Map<String, Object> arbMsg = new HashMap<>();
                    List<Map<String, Object>> msgAmountList = new ArrayList<Map<String, Object>>();
                    JSONArray amountArray = value.getJSONArray("amount");
                    for (int j = 0; j < amountArray.length(); j++) {

                        JSONObject amountOjb = amountArray.getJSONObject(j);
                        Long msgAmount = amountOjb.getLong("amount");
                        String denom = amountOjb.getString("denom");

                        Map<String, Object> amountMap = new HashMap<>();
                        amountMap.put("denom", denom);
                        amountMap.put("amount", msgAmount);
                        msgAmountList.add(amountMap);
                    }

                    if(value.has("to_address")) {
                        String to_address = value.getString("to_address");
                        String from_address = value.getString("from_address");

                        arbMsg.put("to_address", to_address);
                        arbMsg.put("from_address", from_address);
                    } else {
                        String delegator_address = value.getString("delegator_address");
                        String validator_address = value.getString("validator_address");

                        arbMsg.put("delegator_address", delegator_address);
                        arbMsg.put("validator_address", validator_address);
                    }

                    arbMsg.put("amount", msgAmountList);

                    msgMap.put("value", arbMsg);
                    msgList.add(msgMap);
                }

                String memo = testcase.getString("memo");
                Long accountNumber = testcase.getLong("accountNumber");
                String chainId = testcase.getString("chainId");
                Long sequence = testcase.getLong("sequence");

                ImKeyCosmosTransaction transaction = new ImKeyCosmosTransaction(accountNumber, chainId, stdFee, memo, msgList, sequence);

                JSONObject pre = testcase.getJSONObject("preview");
                String paymentDis = pre.getString("payment");
                String receiverDis = pre.getString("receiver");
                String senderDis = pre.getString("sender");
                String feeDis = pre.getString("fee");

                Boolean retry = true;
                int tryCount = 0;
                while(retry) {
                    tryCount ++;
                    try {

                        TransactionSignedResult txSignResult = transaction.signTransaction("0", Path.COSMOS_LEDGER, paymentDis,receiverDis,senderDis,feeDis);
                        JSONObject jsonObject = new JSONObject(txSignResult.getSignedTx());
                        JSONArray signatures = jsonObject.getJSONArray("signatures");
                        JSONObject signatureObj = signatures.getJSONObject(0);
                        String signature = signatureObj.getString("signature");

                        JSONArray signaturesExp = testcase.getJSONArray("signatures");
                        JSONObject signatureExpObj = signaturesExp.getJSONObject(0);
                        String signatureExp = signatureExpObj.getString("signature");

                        if(signature.equals(signatureExp)) {
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

    public static TransactionSignedResult testCosmosTxSign() {


        List<TxMultiSignResult> signResults = new ArrayList<TxMultiSignResult>();

        try {

            /*Coin coin = new Coin("", 0);
            StdFee stdFee = new StdFee(Collections.singletonList(coin), 21906);
            HashMap<String, Object> arbMsg = new HashMap<>();

            arbMsg = new HashMap<>();
            arbMsg.put("delegator_address", "cosmos1y0a8sc5ayv52f2fm5t7hr2g88qgljzk4jcz78f");
            arbMsg.put("validator_address", "cosmosvaloper1zkupr83hrzkn3up5elktzcq3tuft8nxsmwdqgp");
            Map<String, Object> amount = new HashMap<>();
            amount.put("denom", "atom");
            amount.put("amount", "10");

            arbMsg.put("amount", Collections.singletonList(amount));
            Map<String, Object> msgMap = new HashMap<>();
            msgMap.put("type", "cosmos-sdk/MsgDelegate");
            msgMap.put("value", arbMsg);
            ImKeyCosmosTransaction transaction = new ImKeyCosmosTransaction(1, "tendermint_test", stdFee, null, Collections.singletonList(msgMap), 0);

            String to = "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt";
            String from = "cosmos1ajz9y0x3wekez7tz2td2j6l2dftn28v26dd992";
            String payment = "0.001 ATOM";
            String fee = "0.00075 atom";
            TransactionSignedResult txSignResult = transaction.signTransaction("0", , null,to,from,fee);
            return txSignResult;*/


            /*cosmosapi.Cosmos.CosmosTxInput cosmosTxInput = cosmosapi.Cosmos.CosmosTxInput.newBuilder()
                    .setSignData()
                    .setPath(Path.COSMOS_LEDGER)
                    .setToDis(to)
                    .setFromDis(from)
                    .setFeeDis(fee)


            eosapi.Eos.EosSignData data0 = eosapi.Eos.EosSignData.newBuilder()
                    .setTxData("c578065b93aec6a7c811000000000100a6823403ea3055000000572d3ccdcd01000000602a48b37400000000a8ed323225000000602a48b374208410425c95b1ca80969800000000000453595300000000046d656d6f00")
                    .addPubKeys("EOS88XhiiP7Cu5TmAUJqHbyuhyYgd6sei68AU266PyetDDAtjmYWF")
                    .setChainId("aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906")
                    .setTo("bbbb5555bbbb")
                    .setFrom("liujianmin12")
                    .setPayment("undelegatebw 0.0100 EOS")
                    .build();

            eosapi.Eos.EosTxInput eosTxInput = eosapi.Eos.EosTxInput.newBuilder()
                    .setPath(Path.EOS_LEDGER)
                    .addSignDatas(data0)
                    .build();

            Any any = Any.newBuilder()
                    .setValue(eosTxInput.toByteString())
                    .build();


            api.Api.SignParam signParam = api.Api.SignParam.newBuilder()
                    .setChainType("EOS")
                    .setInput(any)
                    .build();

            Any any2 = Any.newBuilder()
                    .setValue(signParam.toByteString())
                    .build();

            api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                    .setMethod("sign_tx")
                    .setParam(any2)
                    .build();
            String hex = NumericUtil.bytesToHex(action.toByteArray());

            // clear_err
            RustApi.INSTANCE.clear_err();

            String result = RustApi.INSTANCE.call_imkey_api(hex);

            String error = RustApi.INSTANCE.get_last_err_message();
            if(!"".equals(error) && null != error) {
                api.Api.Response errorResponse = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
                Boolean isSuccess = errorResponse.getIsSuccess();
                if(!isSuccess) {
                    LogUtil.d("异常： " + errorResponse.getError());

                }
            } else {
                eosapi.Eos.EosTxOutput response = eosapi.Eos.EosTxOutput.parseFrom(ByteUtil.hexStringToByteArray(result));
                String hash = response.getHash();
                List signs = response.getSignsList();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("hash：" + hash);
                LogUtil.d("signs：" + signs.get(0));
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");

                TxMultiSignResult signedResult = new TxMultiSignResult(hash, signs);
                signResults.add(signedResult);
            }*/
        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }
        //return signResults;


        Coin coin = new Coin("", 0);
        StdFee stdFee = new StdFee(Collections.singletonList(coin), 21906);
        HashMap<String, Object> arbMsg = new HashMap<>();

        arbMsg = new HashMap<>();
        arbMsg.put("delegator_address", "cosmos1y0a8sc5ayv52f2fm5t7hr2g88qgljzk4jcz78f");
        arbMsg.put("validator_address", "cosmosvaloper1zkupr83hrzkn3up5elktzcq3tuft8nxsmwdqgp");
        Map<String, Object> amount = new HashMap<>();
        amount.put("denom", "atom");
        amount.put("amount", "10");

        arbMsg.put("amount", Collections.singletonList(amount));
        Map<String, Object> msgMap = new HashMap<>();
        msgMap.put("type", "cosmos-sdk/MsgDelegate");
        msgMap.put("value", arbMsg);
        ImKeyCosmosTransaction transaction = new ImKeyCosmosTransaction(1, "tendermint_test", stdFee, null, Collections.singletonList(msgMap), 0);

        String to = "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt";
        String from = "cosmos1ajz9y0x3wekez7tz2td2j6l2dftn28v26dd992";
        String payment = "0.001 ATOM";
        String fee = "0.00075 atom";
        TransactionSignedResult txSignResult = transaction.signTransaction("0", Path.COSMOS_LEDGER, null,to,from,fee);
        return txSignResult;
    }

    /*public static TransactionSignedResult testCosmosTxSign() {
        Coin coin = new Coin("", 0);
        StdFee stdFee = new StdFee(Collections.singletonList(coin), 21906);
        HashMap<String, Object> arbMsg = new HashMap<>();

        arbMsg = new HashMap<>();
        arbMsg.put("delegator_address", "cosmos1y0a8sc5ayv52f2fm5t7hr2g88qgljzk4jcz78f");
        arbMsg.put("validator_address", "cosmosvaloper1zkupr83hrzkn3up5elktzcq3tuft8nxsmwdqgp");
        Map<String, Object> amount = new HashMap<>();
        amount.put("denom", "atom");
        amount.put("amount", "10");

        arbMsg.put("amount", Collections.singletonList(amount));
        Map<String, Object> msgMap = new HashMap<>();
        msgMap.put("type", "cosmos-sdk/MsgDelegate");
        msgMap.put("value", arbMsg);
        ImKeyCosmosTransaction transaction = new ImKeyCosmosTransaction(1, "tendermint_test", stdFee, null, Collections.singletonList(msgMap), 0);

        String to = "cosmos1yeckxz7tapz34kjwnjxvmxzurerquhtrmxmuxt";
        String from = "cosmos1ajz9y0x3wekez7tz2td2j6l2dftn28v26dd992";
        String payment = "0.001 ATOM";
        String fee = "0.00075 atom";
        TransactionSignedResult txSignResult = transaction.signTransaction("0", Path.COSMOS_LEDGER, null,to,from,fee);
        return txSignResult;
    }*/


}
