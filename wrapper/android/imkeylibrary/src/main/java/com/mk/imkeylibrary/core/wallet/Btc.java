package com.mk.imkeylibrary.core.wallet;

import org.bitcoinj.core.Base58;
import org.bitcoinj.crypto.ChildNumber;

import java.nio.ByteBuffer;
import java.util.List;

import com.google.protobuf.Any;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.wallet.transaction.TransactionSignedResult;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.keycore.RustApi;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class Btc extends Wallet {

    /**
     * @param version mainnet：76067358(0x0488B21E) testnet：70617039(0x043587CF)
     * @param path
     * @return
     */
    public String getXpub(int version, String path) {
        String xpub = null;

        try {

            btcapi.Btc.BtcXpubReq req = btcapi.Btc.BtcXpubReq.newBuilder()
                    .setNetwork("MAINNET")
                    .setPath(path)
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("btc_get_xpub")
                    .setParam(any)
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
                btcapi.Btc.BtcXpubRes response = btcapi.Btc.BtcXpubRes.parseFrom(ByteUtil.hexStringToByteArray(result));
                xpub = response.getXpub();
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");
                LogUtil.d("xpub：" + xpub);
                LogUtil.d("××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××××");

            }

        } catch (Exception e) {
            LogUtil.d("异常：" + e.getMessage());
            e.printStackTrace();
        }

        return xpub;

        /*// path校验
        Path.checkPath(path);

        selectApplet();
        String xpubHex = getXpubHex(path, true);
        String parentXpubHex = getXpubHex(getParentPath(path), true);
        String parentComprsPub = calComprsPub(parentXpubHex.substring(0, 130));

        ByteBuffer ser = ByteBuffer.allocate(78);
        ser.putInt(version);
        ser.put((byte) getDepth(path));
        ser.putInt(getFingerprint(NumericUtil.hexToBytes(parentComprsPub)));
        List<ChildNumber> childNumberList = generatePath(path);
        ser.putInt(childNumberList.get(childNumberList.size() - 1).i());
        ser.put(NumericUtil.hexToBytes(xpubHex.substring(130, 194)));
        ser.put(NumericUtil.hexToBytes(calComprsPub(xpubHex.substring(0, 130))));
        return Base58.encode(addChecksum(ser.array()));*/
    }

    public String getAddress(int version, String path) {

        String address = null;

        try {


            btcapi.Btc.BtcAddressReq req = btcapi.Btc.BtcAddressReq.newBuilder()
                    .setNetwork("MAINNET")
                    .setPath(path)
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("btc_get_address")
                    .setParam(any)
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
                btcapi.Btc.BtcAddressRes response = btcapi.Btc.BtcAddressRes.parseFrom(ByteUtil.hexStringToByteArray(result));
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
        /*
        // path校验
        Path.checkPath(path);

        selectApplet();
        String xpub = getXpubHex(path, true);
        String comprsPub = calComprsPub(xpub.substring(0, 130));
        return pub2Address(version, comprsPub);*/
    }

    public String displayAddress(int version, String path) {

        String address = null;

        try {

            btcapi.Btc.BtcAddressReq req = btcapi.Btc.BtcAddressReq.newBuilder()
                    .setNetwork("MAINNET")
                    .setPath(path)
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("btc_register_address")
                    .setParam(any)
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
                btcapi.Btc.BtcAddressRes response = btcapi.Btc.BtcAddressRes.parseFrom(ByteUtil.hexStringToByteArray(result));
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
        /*
        // path校验
        Path.checkPath(path);

        String mainAddr = getAddress(version, path);

        String apduCoinReg = Apdu.btcCoinReg(mainAddr.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);
        return mainAddr;*/
    }

    public String getSegWitAddress(int version, String path) {

        String address = null;

        try {

            btcapi.Btc.BtcAddressReq req = btcapi.Btc.BtcAddressReq.newBuilder()
                    .setNetwork("MAINNET")
                    .setPath(path)
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("btc_get_setwit_address")
                    .setParam(any)
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
                btcapi.Btc.BtcAddressRes response = btcapi.Btc.BtcAddressRes.parseFrom(ByteUtil.hexStringToByteArray(result));
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
/*

        // path校验
        Path.checkPath(path);

        selectApplet();
        if (version < 0 || version >= 256) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }
        String xpub = getXpubHex(path, true);
        String comprsPub = calComprsPub(xpub.substring(0, 130));
        return calcSegWitAddress(version, comprsPub);*/
    }

    public String displaySegWitAddress(int version, String path) {

        String address = null;

        try {

            btcapi.Btc.BtcAddressReq req = btcapi.Btc.BtcAddressReq.newBuilder()
                    .setNetwork("MAINNET")
                    .setPath(path)
                    .build();

            Any any = Any.newBuilder()
                    .setValue(req.toByteString())
                    .build();

            api.Api.ImkeyAction action = api.Api.ImkeyAction.newBuilder()
                    .setMethod("btc_register_segwit_address")
                    .setParam(any)
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
                btcapi.Btc.BtcAddressRes response = btcapi.Btc.BtcAddressRes.parseFrom(ByteUtil.hexStringToByteArray(result));
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

        /*
        // path校验
        Path.checkPath(path);

        String mainAddr = getSegWitAddress(version, path);

        String apduCoinReg = Apdu.btcCoinReg(mainAddr.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);
        return mainAddr;*/
    }

    @Override
    protected String getAid() {
        return Applet.BTC_AID;
    }
}
