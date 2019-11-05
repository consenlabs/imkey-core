package com.mk.imkeylibrary.core;

import java.util.ArrayList;
import java.util.List;

import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;

public class Apdu {
    private static final int LC_MAX = 245;// max length of data in apdu
    public static final int Hash_ALL = 0x01;


    public static String select(String aid) {
        byte[] AID = ByteUtil.hexStringToByteArray(aid);
        byte[] header = header((byte) 0x00, (byte) 0xA4, (byte) 0x04, (byte) 0x00, (byte) AID.length);
        byte[] selectApdu = new byte[header.length + AID.length];
        System.arraycopy(header, 0, selectApdu, 0, header.length);
        System.arraycopy(AID, 0, selectApdu, header.length, AID.length);
        return ByteUtil.byteArrayToHexString(selectApdu);
    }

    private static byte[] header(byte cla, byte ins, byte p1, byte p2, byte lc) {
        return new byte[]{cla, ins, p1, p2, lc};
    }

    private static List<String> prepare(int ins, String data) {
        List<String> list = new ArrayList<>();
        byte[] bytes = ByteUtil.hexStringToByteArray(data);
        int size = bytes.length / LC_MAX + (bytes.length % LC_MAX == 0 ? 0 : 1);
        for (int i = 0; i < size; i++) {
            byte[] apdu;
            if (i == size - 1) {
                apdu = new byte[bytes.length - LC_MAX * (size - 1) + 6];
            } else {
                apdu = new byte[LC_MAX + 6];
            }

            apdu[0] = (byte) 0x80;//CLA
            apdu[1] = (byte) ins;//INS
            //P1
            if (i == 0) {
                apdu[2] = (byte) 0x00;
            } else {
                apdu[2] = (byte) 0x80;
            }
            if (i == size - 1) {
                apdu[3] = (byte) 0x80;//P2
                apdu[4] = (byte) (bytes.length - LC_MAX * (size - 1));//Lc
                System.arraycopy(bytes, i * LC_MAX, apdu, 5, bytes.length - LC_MAX * i);//Data
            } else {
                apdu[3] = (byte) 0x00;//P2
                apdu[4] = (byte) 0xF5;//Lc
                System.arraycopy(bytes, i * LC_MAX, apdu, 5, LC_MAX);//Data
            }
            apdu[apdu.length - 1] = (byte) 0x00;//Le
            list.add(ByteUtil.byteArrayToHexString(apdu));
        }
        return list;
    }

    private static String sign(int ins, int index, int hashType, String path) {
        byte[] bytes = path.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) ins;//INS
        apdu[2] = (byte) index;//P1
        apdu[3] = (byte) hashType;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static List<String> btcPrepare(String data) {
        return prepare(0x41, data);
    }

    public static String btcSign(int index, int hashType, String path) {
        return sign(0x42, index, hashType, path);
    }

    public static List<String> btcSegwitPrepare(String data) {
        return prepare(0x31, data);
    }

    public static String btcXpub(String path, boolean verifyKey) {
        byte[] bytes = path.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x43;//INS
        if(verifyKey) {
            apdu[2] = (byte) 0x01;//P1
        } else {
            apdu[2] = (byte) 0x00;//P1
        }
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static List<String> ethPrepare(String data) {
        return prepare(0x51, data);
    }

    public static String ethSign(String path) {
        return sign(0x52, 0x00, 0x00, path);
    }

    public static String cosmosSign(String path) {
        return sign(0x72, 0x00, 0x00, path);
    }

    public static String ethXpub(String path, boolean verifyKey) {
        byte[] bytes = path.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x53;//INS
        if(verifyKey) {
            apdu[2] = (byte) 0x01;//P1
        } else {
            apdu[2] = (byte) 0x00;//P1
        }
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static List<String> ethMsgPrepare(String data) {
        return prepare(0x54, data);
    }

    public static String ethMsgSign(String path) {
        byte[] bytes = path.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x55;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }


    public static List<String> eosPrepare(String data) {
        return prepare(0x61, data);
    }
    public static List<String> cosmosPrepare(String data) {
        return prepare(0x71, data);
    }

    public static String eosTxSign(int nonce) {
        byte[] apdu = new byte[8];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x62;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) 2;//Lc
        apdu[5] = (byte) ((nonce & 0xFF00) >> 8);
        apdu[6] = (byte) (nonce & 0x00FF);
        apdu[7] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static List<String> eosMsgPrepare(String data) {
        return prepare(0x64, data);
    }

    public static String eosMsgSign(int nonce) {
        byte[] apdu = new byte[8];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x65;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) 2;//Lc
        apdu[5] = (byte) ((nonce & 0xFF00) >> 8);
        apdu[6] = (byte) (nonce & 0x00FF);
        apdu[7] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String eosPub(String path, boolean verifyKey) {
        byte[] bytes = path.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x63;//INS
        if(verifyKey) {
            apdu[2] = (byte) 0x01;//P1
        } else {
            apdu[2] = (byte) 0x00;//P1
        }
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String cosmosPub(String path, boolean verifyKey) {
        byte[] bytes = path.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x73;//INS
        if(verifyKey) {
            apdu[2] = (byte) 0x01;//P1
        } else {
            apdu[2] = (byte) 0x00;//P1
        }
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String getResponseData(String response) {
        return response.substring(0, response.length() - 4);
    }

    public static String btcCoinReg(byte[] addr) {
        byte[] apdu = new byte[6 + addr.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x36;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) (addr.length);//Lc
        System.arraycopy(addr, 0, apdu, 5, addr.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String ethCoinReg(byte[] addr) {
        byte[] apdu = new byte[6 + addr.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x56;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) addr.length;//Lc
        System.arraycopy(addr, 0, apdu, 5, addr.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String eosCoinReg(byte[] pubKey) {
        byte[] apdu = new byte[6 + pubKey.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x66;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) pubKey.length;//Lc
        System.arraycopy(pubKey, 0, apdu, 5, pubKey.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String cosmosCoinReg(byte[] addr) {
        byte[] apdu = new byte[6 + addr.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x76;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) addr.length;//Lc
        System.arraycopy(addr, 0, apdu, 5, addr.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static void checkResponse(String res) {
        if(res.endsWith(Constants.APDU_RSP_SUCCESS)) {
            return;
        }

        if(res.equals(Constants.APDU_RSP_USER_NOT_CONFIRMED)) {
            throw new ImkeyException(Messages.IMKEY_USER_NOT_CONFIRMED);
        } else if(res.equals(Constants.APDU_CONDITIONS_NOT_SATISFIED)) {
            throw new ImkeyException(Messages.IMKEY_CONDITIONS_NOT_SATISFIED);
        } else if(res.equals(Constants.APDU_RSP_INCORRECT_P1P2)) {
            throw new ImkeyException(Messages.IMKEY_COMMAND_FORMAT_ERROR);
        } else if(res.equals(Constants.APDU_RSP_CLA_NOT_SUPPORTED)) {
            throw new ImkeyException(Messages.IMKEY_COMMAND_FORMAT_ERROR);
        } else if(res.equals(Constants.APDU_RSP_APPLET_WRONG_DATA)) {
            throw new ImkeyException(Messages.IMKEY_COMMAND_DATA_ERROR);
        } else if(res.equals(Constants.APDU_RSP_APPLET_NOT_EXIST)) {
            throw new ImkeyException(Messages.IMKEY_APPLET_NOT_EXIST);
        } else if(res.equals(Constants.APDU_RSP_WRONG_LENGTH)) {
            throw new ImkeyException(Messages.IMKEY_APDU_WRONG_LENGTH);
        } else if(res.equals(Constants.APDU_RSP_SIGNATURE_VERIFY_FAILED)) {
            throw new ImkeyException(Messages.IMKEY_SIGNATURE_VERIFY_FAIL);
        } else if(res.equals(Constants.APDU_BLUETOOTH_CHANNEL_ERROR)) {
            throw new ImkeyException(Messages.IMKEY_BLUETOOTH_CHANNEL_ERROR);
        } else if(res.equals(Constants.APDU_RSP_FUNCTION_NOT_SUPPORTED)) {
            throw new ImkeyException(Messages.IMKEY_APPLET_FUNCTION_NOT_SUPPORTED);
        } else if(res.equals(Constants.APDU_RSP_EXCEEDED_MAX_UTXO_NUMBER)) {
            throw new ImkeyException(Messages.IMKEY_EXCEEDED_MAX_UTXO_NUMBER);
        } else {
            throw new ImkeyException(Messages.IMKEY_COMMAND_EXECUTE_FAIL + "_" + res);
        }

    }

    public static void checkImKeyStatus(String res) {

        if(res.equals(Constants.APDU_RSP_WALLET_NOT_CREATED)){
            throw new ImkeyException(Messages.IMKEY_WALLET_NOT_CREATED);
        }
        if(res.equals(Constants.APDU_RSP_IN_MENU_PAGE)){
            throw new ImkeyException(Messages.IMKEY_IN_MENU_PAGE);
        }
        if(res.equals(Constants.APDU_RSP_PIN_NOT_VERIFIED)){
            throw new ImkeyException(Messages.IMKEY_PIN_NOT_VERIFIED);
        }
    }

    public static String setBleName(String bleName){
        byte[] bytes = bleName.getBytes();
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0xFF;//CLA
        apdu[1] = (byte) 0xDA;//INS
        apdu[2] = (byte) 0x46;//P1
        apdu[3] = (byte) 0x54;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String bindCheck(byte[] bytes) {
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x71;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String generateAuthCode() {
        byte[] apdu = new byte[5];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x72;//INS
        apdu[2] = (byte) 0x00;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) 0x00;;//Lc
        return ByteUtil.byteArrayToHexString(apdu);
    }

    public static String identityVerify(byte P1, byte[] bytes) {
        byte[] apdu = new byte[6 + bytes.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x73;//INS
        apdu[2] = P1;//P1
        apdu[3] = (byte) 0x00;//P2
        apdu[4] = (byte) bytes.length;//Lc
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    /**
     * BTC_TRX_SIGN_PREPARE
     * @param p1 00:output data
     *           80:input data
     * @param data
     * @return
     */
    public static String btcPrepareInput(byte p1, String data) {
        byte[] bytes = ByteUtil.hexStringToByteArray(data);
        byte[] apdu = new byte[bytes.length + 6];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x41;//INS
        apdu[2] = p1;//P1
        apdu[3] = (byte) 0x00;//p2
        apdu[4] = (byte) bytes.length;
        //copy data to apdu
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        return ByteUtil.byteArrayToHexString(apdu);
    }

    /**
     * BTC_SEGWIT_TRX_PREPARE
     * @param p1 00:output data
     *           04:hash and vout data
     *           80:sequence data
     * @param data
     * @return
     */
    public static List<String> btcSegwitPrepare(byte p1, final byte[] data){
        List<String> apduList = new ArrayList<>();
        int size = (data.length - 1) / LC_MAX + 1;
        for(int i = 0; i < size; i++){
            if (i == size - 1) {
                int len = (data.length % LC_MAX == 0)?LC_MAX:(data.length % LC_MAX);
                byte[] apdu = new byte[len + 6];
                apdu[0] = (byte) 0x80;
                apdu[1] = (byte) 0x31;
                apdu[2] = p1;
                apdu[3] = (byte) 0x80;
                apdu[4] = (byte) len;
                System.arraycopy(data, i * LC_MAX, apdu, 5, len);
                apduList.add(ByteUtil.byteArrayToHexString(apdu));
            } else {
                byte[] apdu = new byte[LC_MAX + 6];
                apdu[0] = (byte) 0x80;
                apdu[1] = (byte) 0x31;
                apdu[2] = p1;
                apdu[3] = (byte) 0x00;
                apdu[4] = (byte) LC_MAX;
                System.arraycopy(data, i * LC_MAX, apdu, 5, LC_MAX);
                apduList.add(ByteUtil.byteArrayToHexString(apdu));
            }
        }
        return apduList;
    }

    /**
     * BTC_SEGWIT_TRX_SIGN
     * @param hashType 01: ALL
     * @param data
     * @return
     */
    public static String btcSegwitSign(Boolean lastOne, int hashType, byte[] data) {
        byte[] apdu = new byte[6 + data.length];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x32;//INS
        apdu[2] = lastOne ? (byte) 0x80 : (byte) 0x00;//P1
        apdu[3] = (byte) hashType;//P2
        apdu[4] = (byte) data.length;//Lc
        System.arraycopy(data, 0, apdu, 5, data.length);
        apdu[apdu.length - 1] = (byte) 0x00;//Le
        return ByteUtil.byteArrayToHexString(apdu);
    }

    /**
     * OMNI_TRX_SIGN_PREPARE
     * @param p1 00:output data
     *           80:input data
     * @param data
     * @return
     */
    public static String omniPrepareData(byte p1, String data) {
        byte[] bytes = ByteUtil.hexStringToByteArray(data);

        byte[] apdu = new byte[bytes.length + 6];
        apdu[0] = (byte) 0x80;//CLA
        apdu[1] = (byte) 0x44;//INS
        //P1
        apdu[2] = p1;
        //p2
        apdu[3] = (byte) 0x00;
        apdu[4] = (byte) bytes.length;
        //copy data to apdu
        System.arraycopy(bytes, 0, apdu, 5, bytes.length);
        return ByteUtil.byteArrayToHexString(apdu);
    }

    /**
     * OMNI_SEGWIT_TRX_PREPARE
     * @param p1 00:output data
     *           04:hash and vout data
     *           08:sequence data
     * @param data
     * @return
     */
    public static List<String> omniSegwitPrepare(byte p1, final byte[] data){
        List<String> apduList = new ArrayList<>();
        int size = (data.length - 1) / LC_MAX + 1;
        for(int i = 0; i < size; i++){
            if (i == size - 1) {
                int len = (data.length % LC_MAX == 0)?LC_MAX:(data.length % LC_MAX);
                byte[] apdu = new byte[len + 6];
                apdu[0] = (byte) 0x80;
                apdu[1] = (byte) 0x34;
                apdu[2] = p1;
                apdu[3] = (byte) 0x80;
                apdu[4] = (byte) len;
                System.arraycopy(data, i * LC_MAX, apdu, 5, len);
                apduList.add(ByteUtil.byteArrayToHexString(apdu));
            } else {
                byte[] apdu = new byte[LC_MAX + 6];
                apdu[0] = (byte) 0x80;
                apdu[1] = (byte) 0x34;
                apdu[2] = p1;
                apdu[3] = (byte) 0x00;
                apdu[4] = (byte) LC_MAX;
                System.arraycopy(data, i * LC_MAX, apdu, 5, LC_MAX);
                apduList.add(ByteUtil.byteArrayToHexString(apdu));
            }
        }
        return apduList;
    }

}
