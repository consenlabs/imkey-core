package com.mk.imkeylibrary.core.wallet.transaction;


import com.subgraph.orchid.encoders.Hex;

import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.Sha256Hash;

import java.math.BigInteger;
import java.nio.charset.Charset;
import java.security.SignatureException;
import java.util.Arrays;
import java.util.List;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.wallet.Eth;
import com.mk.imkeylibrary.core.wallet.Wallet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

import static com.google.common.base.Preconditions.checkState;

//import org.consenlabs.tokencore.wallet.address.EthereumAddressCreator; //@XM@20170705 not needed now

/**
 * Created by xyz on 2017/12/20.
 */

public class EthereumSign {
/* @XM@20181012 deprecated
    public static String personalSign(Context context, BleDevice bleDevice, String data, byte[] prvKeyBytes) throws BleException {
        byte[] dataBytes = dataToBytes(data);
        int msgLen = dataBytes.length;
        String headerMsg = String.format(Locale.ENGLISH, "\u0019Ethereum Signed Message:\n%d", msgLen);
        byte[] headerMsgBytes = headerMsg.getBytes(Charset.forName("UTF-8"));
        byte[] dataToSign = ByteUtil.concat(headerMsgBytes, dataBytes);
        return signPersonalMessage(context, bleDevice, dataToSign, prvKeyBytes).toString();
    }

    public static String sign(Context context, BleDevice bleDevice, String data, byte[] prvKeyBytes) throws BleException {
        return signPersonalMessage(context, bleDevice, dataToBytes(data), prvKeyBytes).toString();
    }
*/
    public static BigInteger ecRecover(String data, String signature) throws SignatureException {
        byte[] msgBytes = dataToBytes(data);
        signature = NumericUtil.cleanHexPrefix(signature);
        byte[] r = Hex.decode(signature.substring(0, 64));
        byte[] s = Hex.decode(signature.substring(64, 128));
        int receiveId = Integer.valueOf(signature.substring(128), 16);
        SignatureData signatureData = new SignatureData((byte) receiveId, r, s);

        return signedMessageToKey(msgBytes, signatureData);
    }

    /* @XM@20170705 not needed now
      public static String recoverAddress(String data, String signature) {
        try {
          BigInteger pubKey = ecRecover(data, signature);
          return new EthereumAddressCreator().fromPublicKey(pubKey);
        } catch (SignatureException e) {
          return "";
        }
      }
    */
    private static byte[] dataToBytes(String data) {
        byte[] messageBytes;
        if (NumericUtil.isValidHex(data)) {
            messageBytes = NumericUtil.hexToBytes(data);
        } else {
            messageBytes = data.getBytes(Charset.forName("UTF-8"));
        }
        return messageBytes;
    }

    static SignatureData signMessage(byte[] message, byte[] path, byte[] payment, byte[] receiver, byte[] fee, byte[] sender) {
        byte[] messageWtl = ByteUtil.concat(new byte[]{(byte)0x01},
                            ByteUtil.concat(new byte[]{(byte)((message.length & 0xFF00) >> 8),(byte)(message.length & 0x00FF)}, message));
        byte[] txPack = ByteUtil.concat(messageWtl, ByteUtil.concat(payment, ByteUtil.concat(receiver, fee)));
        byte[] hashData  = Sha256Hash.hashTwice(txPack);
        byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
        byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signature.length},signature));
        byte[] apduPack = ByteUtil.concat(signatureWtl, txPack);

        List<String> prepares = Apdu.ethPrepare(NumericUtil.bytesToHex(apduPack));
        for (String apdu : prepares) {
            String res = Ble.getInstance().sendApdu(apdu, Constants.SEND_SIGN_PRE_APDU_TIMEOUT);
            Apdu.checkResponse(res);
        }

        String strPath = new String(path);

        // 获取公钥和地址
        String getPubKeyRes = new Eth().getEthXpubHex(strPath,false);
        String mainAddr = Wallet.publicKeyToAddress(NumericUtil.hexToBytes(getPubKeyRes.substring(2, 130)));
        byte[] checkedAddr  = Wallet.checkedEthAddress(mainAddr).getBytes();

        if (Arrays.equals(sender, checkedAddr)) {
            // sign
            String apdu = Apdu.ethSign(strPath);
            String signRes = Ble.getInstance().sendApdu(apdu);
            Apdu.checkResponse(signRes);

            String r = signRes.substring(2, 66);
            String s = signRes.substring(66, 130);


            ECKey ecKey = ECKey.fromPublicOnly(NumericUtil.hexToBytes(getPubKeyRes.substring(0, 130)));

            ECKey.ECDSASignature sig = new ECKey.ECDSASignature(new BigInteger(r, 16), new BigInteger(s, 16)).toCanonicalised();
            byte[] messageHash = Hash.keccak256(message);
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

            return new SignatureData(v, rByte, sByte);
        } else {
            throw new ImkeyException(Messages.IMKEY_ADDRESS_MISMATCH_WITH_PATH);
        }

        //int headerByte = byteV[0] + 27;
        //return new SignatureData((byte)headerByte, byteR, byteS);   //@XM@20180904 seems missing +27..
//        //below code are not used
//        ECKey ecKey = ECKey.fromPrivate(path);
//        byte[] messageHash = Hash.keccak256(message);
//        return signAsRecoverable(messageHash, ecKey);
    }


    /**
     * Given an arbitrary piece of text and an Ethereum message signature encoded in bytes,
     * returns the public key that was used to sign it. This can then be compared to the expected
     * public key to determine if the signature was correct.
     *
     * @param message       RLP encoded message.
     * @param signatureData The message signature components
     * @return the public key used to sign the message
     * @throws SignatureException If the public key could not be recovered or if there was a
     *                            signature format error.
     */
    private static BigInteger signedMessageToKey(byte[] message, SignatureData signatureData) throws SignatureException {

        byte[] r = signatureData.getR();
        byte[] s = signatureData.getS();
        checkState(r != null && r.length == 32, "r must be 32 bytes");
        checkState(s != null && s.length == 32, "s must be 32 bytes");

        int header = signatureData.getV() & 0xFF;
        // The header byte: 0x1B = first key with even y, 0x1C = first key with odd y,
        //                  0x1D = second key with even y, 0x1E = second key with odd y
        if (header < 27 || header > 34) {
            throw new SignatureException("Header byte out of range: " + header);
        }

        ECKey.ECDSASignature sig = new ECKey.ECDSASignature(
                new BigInteger(1, signatureData.getR()),
                new BigInteger(1,signatureData.getS()));

        byte[] messageHash = Hash.keccak256(message);
        int recId = header - 27;
        ECKey key = ECKey.recoverFromSignature(recId, sig, Sha256Hash.wrap(messageHash), false);
        if (key == null) {
            throw new SignatureException("Could not recover public key from signature");
        }
        byte[] pubKeyBytes = key.getPubKeyPoint().getEncoded(false);
        return NumericUtil.bytesToBigInteger(Arrays.copyOfRange(pubKeyBytes, 1, pubKeyBytes.length));
    }

    public static SignatureData signAsRecoverable(byte[] value, ECKey ecKey) {

        ECKey.ECDSASignature sig = ecKey.sign(Sha256Hash.wrap(value));

        // Now we have to work backwards to figure out the recId needed to recover the signature.
        int recId = -1;
        for (int i = 0; i < 4; i++) {
            ECKey recoverKey = ECKey.recoverFromSignature(i, sig, Sha256Hash.wrap(value), false);
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
        byte[] r = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.r, 32);
        byte[] s = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.s, 32);

        return new SignatureData(v, r, s);
    }

}
