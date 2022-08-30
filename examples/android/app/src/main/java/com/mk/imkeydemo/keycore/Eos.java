package com.mk.imkeydemo.keycore;

import com.google.protobuf.Any;
import com.mk.imkeydemo.utils.NumericUtil;

import java.nio.charset.Charset;

import api.Api;
import im.imkey.imkeylibrary.utils.ByteUtil;
import im.imkey.imkeylibrary.utils.LogUtil;

public class Eos extends Wallet {

    public String getPubKey(String path) {


        String eosPK = null;

        try {

            Api.PubKeyParam req = Api.PubKeyParam.newBuilder()
                    .setPath(path)
                    .setChainType("EOS")
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("get_pub_key")
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
                Api.PubKeyResult response = Api.PubKeyResult.parseFrom(ByteUtil.hexStringToByteArray(result));
                eosPK = response.getPubKey();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("eosPK：" + eosPK);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");

            }

        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }

        return eosPK;


        /*// path校验
        Path.checkPath(path);

        selectApplet();
        String xpub = getEosXpubHex(path, true);
        String comprsPub = calComprsPub(xpub);
        byte[] pubKeyData = NumericUtil.hexToBytes(comprsPub);
        RIPEMD160Digest digest = new RIPEMD160Digest();
        digest.update(pubKeyData, 0, pubKeyData.length);
        byte[] out = new byte[20];
        digest.doFinal(out, 0);
        byte[] checksumBytes = Arrays.copyOfRange(out, 0, 4);

        pubKeyData = ByteUtil.concat(pubKeyData, checksumBytes);
        String eosPK = "EOS" + Base58.encode(pubKeyData);
        return eosPK;*/
    }

    public String displayPubKey(String path) {

        String eosPK = null;

        try {

            Api.PubKeyParam req = Api.PubKeyParam.newBuilder()
                    .setPath(path)
                    .setChainType("EOS")
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("eos_register_pubkey")
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
                Api.PubKeyResult response = Api.PubKeyResult.parseFrom(ByteUtil.hexStringToByteArray(result));
                eosPK = response.getPubKey();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("eosPK：" + eosPK);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");

            }

        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }

        return eosPK;
        /*
        // path校验
        Path.checkPath(path);

        String eosPK = getPubKey(path);

        String apduCoinReg = Apdu.eosCoinReg(eosPK.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);

        return eosPK;*/
    }

    protected String getAid() {
        return Applet.EOS_AID;
    }

//    public static String eosEcSign(String data, boolean isHex, String pubKey, String path) {
//        // path校验
//        Path.checkPath(path);
//
//        byte[] dataHashed;
//        if (isHex) {
//            dataHashed = NumericUtil.hexToBytes(data);
//        } else {
//            byte[] dataBytes = data.getBytes(Charset.forName("UTF-8"));
//            dataHashed = Hash.sha256(dataBytes);
//        }
//        byte[] dataHashPrefix = {0x01,0x20};  //TL
//        byte[] pathBytes = path.getBytes();
//        byte[] pathPrefix = {0x02,(byte) pathBytes.length};  //TL;
//        byte[] txPack = ByteUtil.concat(ByteUtil.concat(dataHashPrefix, dataHashed), ByteUtil.concat(pathPrefix, pathBytes));
//        String signed = EOSSign.signMessage(txPack, pubKey, dataHashed);
//        return signed;
//    }

}
