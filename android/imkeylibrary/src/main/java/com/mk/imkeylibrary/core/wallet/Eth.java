package com.mk.imkeylibrary.core.wallet;

import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.Sha256Hash;

import java.math.BigInteger;
import java.nio.charset.Charset;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;

import com.google.protobuf.Any;
import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.wallet.transaction.SignatureData;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.keycore.RustApi;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;


public class Eth extends Wallet {

    public String getAddress(String path) {

        String address = null;

        try {

            api.Api.AddressParam addressParam = api.Api.AddressParam.newBuilder()
                    .setChainType("ETH")
                    .setPath(path)
                    .build();

            Any any2 = Any.newBuilder()
                    .setValue(addressParam.toByteString())
                    .build();

            api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                    .setMethod("get_address")
                    .setParam(any2)
                    .build();
            String hex = NumericUtil.bytesToHex(action.toByteArray());

            // clear_err
            RustApi.INSTANCE.clear_err();

            String result = RustApi.INSTANCE.call_tcx_api(hex);

            String error = RustApi.INSTANCE.get_last_err_message();
            if(!"".equals(error) && null != error) {
                api.Api.Response errorResponse = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
                Boolean isSuccess = errorResponse.getIsSuccess();
                if(!isSuccess) {
                    LogUtil.d("异常： " + errorResponse.getError());

                }
            } else {
                ethapi.Eth.EthAddressResponse response = ethapi.Eth.EthAddressResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
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

        /*// path校验
        Path.checkPath(path);

        selectApplet();
        String xpub = getEthXpubHex(path, true);
        return publicKeyToAddress(NumericUtil.hexToBytes(xpub.substring(2, 130)));*/
    }

    public String displayAddress(String path) {

        String address = null;

        try {

            api.Api.AddressParam addressParam = api.Api.AddressParam.newBuilder()
                    .setChainType("ETH")
                    .setPath(path)
                    .build();

            Any any2 = Any.newBuilder()
                    .setValue(addressParam.toByteString())
                    .build();

            api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                    .setMethod("register_coin")
                    .setParam(any2)
                    .build();
            String hex = NumericUtil.bytesToHex(action.toByteArray());

            // clear_err
            RustApi.INSTANCE.clear_err();

            String result = RustApi.INSTANCE.call_tcx_api(hex);

            String error = RustApi.INSTANCE.get_last_err_message();
            if(!"".equals(error) && null != error) {
                api.Api.Response errorResponse = api.Api.Response.parseFrom(ByteUtil.hexStringToByteArray(error));
                Boolean isSuccess = errorResponse.getIsSuccess();
                if(!isSuccess) {
                    LogUtil.d("异常： " + errorResponse.getError());

                }
            } else {
                ethapi.Eth.EthAddressResponse response = ethapi.Eth.EthAddressResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
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

        String mainAddr = getAddress(path);
        String apduCoinReg = Apdu.ethCoinReg(Wallet.checkedEthAddress(mainAddr).getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);
        return mainAddr;*/
    }

    public String signPersonalMessage(String path, String message, String sender) {
        // path校验
        Path.checkPath(path);
        
        selectApplet();
        byte[] dataBytes = dataToBytes(message);
        int msgLen = dataBytes.length;
        String headerMsg = String.format(Locale.ENGLISH, "\u0019Ethereum Signed Message:\n%d", msgLen);
        byte[] headerMsgBytes = headerMsg.getBytes(Charset.forName("UTF-8"));
        byte[] dataToSign = ByteUtil.concat(headerMsgBytes, dataBytes);

        //byte[] dataToSignWtl = ByteUtil.concat(new byte[]{(byte)0x01}, ByteUtil.concat(new byte[]{(byte)dataToSign.length},dataToSign));
        byte[] dataToSignWtl = ByteUtil.concat(new byte[]{(byte)0x01},
                ByteUtil.concat(new byte[]{(byte)((dataToSign.length & 0xFF00) >> 8),(byte)(dataToSign.length & 0x00FF)}, dataToSign));


        byte[] hashData  = Sha256Hash.hashTwice(dataToSignWtl);
        byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
        byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signature.length},signature));
        byte[] apduPack = ByteUtil.concat(signatureWtl, dataToSignWtl);

        // 获取公钥
        String getXpubApdu = Apdu.ethXpub(path, false);
        String res = sendApdu(getXpubApdu);
        Apdu.checkResponse(res);

        String getPubKeyRes = res.substring(0, res.length() - 4);
        String mainAddr = Wallet.publicKeyToAddress(NumericUtil.hexToBytes(getPubKeyRes.substring(2, 130)));
        byte[] checkedAddr  = Wallet.checkedEthAddress(mainAddr).getBytes();

        if (Arrays.equals(checkedAddr, sender.getBytes())) {
            List<String> apdus = Apdu.ethMsgPrepare(ByteUtil.byteArrayToHexString(apduPack));
            for (int i = 0; i < apdus.size(); i++) {
                res = Ble.getInstance().sendApdu(apdus.get(i), Constants.SEND_SIGN_PRE_APDU_TIMEOUT);
                Apdu.checkResponse(res);
            }

            String signApdu = Apdu.ethMsgSign(path);
            String signRes = sendApdu(signApdu);
            Apdu.checkResponse(signRes);

            String r = signRes.substring(2, 66);
            String s = signRes.substring(66, 130);


            ECKey ecKey = ECKey.fromPublicOnly(NumericUtil.hexToBytes(getPubKeyRes.substring(0, 130)));

            ECKey.ECDSASignature sig = new ECKey.ECDSASignature(new BigInteger(r, 16), new BigInteger(s, 16)).toCanonicalised();
            String messageHash = Hash.keccak256(ByteUtil.byteArrayToHexString(dataToSign));
            int recId = -1;

            for (int i = 0; i < 4; i++) {
                ECKey recoverKey = ECKey.recoverFromSignature(i, sig, Sha256Hash.wrap(messageHash), false);
                if (recoverKey != null && recoverKey.getPubKeyPoint().equals(ecKey.getPubKeyPoint())) {
                    recId = i;
                    break;
                }
            }
            if (recId == -1) {
                throw new RuntimeException(
                        "Could not construct a recoverable key. This should never happen.");
            }

            int headerByte = recId + 27;
            // 1 header + 32 bytes for R + 32 bytes for S
            byte v = (byte) headerByte;
            byte[] rByte = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.r, 32);
            byte[] sByte = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.s, 32);

            return new SignatureData(v, rByte, sByte).toString();
        } else {
            throw new ImkeyException(Messages.IMKEY_ADDRESS_MISMATCH_WITH_PATH);
        }
    }

    @Override
    protected String getAid() {
        return Applet.ETH_AID;
    }

    private static byte[] dataToBytes(String data) {
        byte[] messageBytes;
        if (NumericUtil.isValidHex(data)) {
            messageBytes = NumericUtil.hexToBytes(data);
        } else {
            messageBytes = data.getBytes(Charset.forName("UTF-8"));
        }
        return messageBytes;
    }
}
