package com.mk.imkeylibrary.core.wallet;

import org.bitcoinj.core.Base58;
import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.Sha256Hash;
import org.bitcoinj.core.Utils;
import org.bitcoinj.crypto.ChildNumber;

import java.math.BigInteger;
import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.device.key.KeyManager;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class Wallet {

    protected String getAid(){
        throw new RuntimeException("need overwirte this method");
    }

    protected String sendApdu(String apdu) {
        return Ble.getInstance().sendApdu(apdu);
    }

    protected void selectApplet() {
        String selectApdu = Apdu.select(getAid());
        String res = sendApdu(selectApdu);
        Apdu.checkResponse(res);
    }

    public String getXpubHex(String path, boolean verifyKey) {
        String getXpubApdu = Apdu.btcXpub(path, verifyKey);
        String res = sendApdu(getXpubApdu);
        Apdu.checkResponse(res);

        // 65字节pub+32字节chaincode+签名值
        String signature = res.substring(97*2, res.length()-4);
        String data = res.substring(0, 97*2);

        Boolean verifyRes = Wallet.signVerify(ByteUtil.hexStringToByteArray(data), ByteUtil.hexStringToByteArray(signature));
        if(!verifyRes) {
            throw new ImkeyException(Messages.IMKEY_SIGNATURE_VERIFY_FAIL);
        }

        return res.substring(0, 194);
    }

    public String getEthXpubHex(String path, boolean verifyKey) {

        String getXpubApdu = Apdu.ethXpub(path, verifyKey);
        String res = sendApdu(getXpubApdu);
        Apdu.checkResponse(res);

        // 65字节pub+32字节chaincode+签名值
        String signature = res.substring(97*2, res.length()-4);
        String data = res.substring(0, 97*2);

        Boolean verifyRes = Wallet.signVerify(ByteUtil.hexStringToByteArray(data), ByteUtil.hexStringToByteArray(signature));
        if(!verifyRes) {
            throw new ImkeyException(Messages.IMKEY_SIGNATURE_VERIFY_FAIL);
        }

        return res.substring(0, 130);
    }

    public String getEosXpubHex(String path, boolean verifyKey) {

        String getXpubApdu = Apdu.eosPub(path, verifyKey);
        String res = sendApdu(getXpubApdu);
        Apdu.checkResponse(res);

        // 65字节pub+32字节chaincode+签名值
        String signature = res.substring(97*2, res.length()-4);
        String data = res.substring(0, 97*2);

        Boolean verifyRes = Wallet.signVerify(ByteUtil.hexStringToByteArray(data), ByteUtil.hexStringToByteArray(signature));
        if(!verifyRes) {
            throw new ImkeyException(Messages.IMKEY_SIGNATURE_VERIFY_FAIL);
        }
        /*
        if(verifyKey) {
            // 验证签名
            String r = res.substring(196, 260);
            String s = res.substring(260, 324);
            TransactionSignature signature = new TransactionSignature(new BigInteger(r,16), new BigInteger(s,16));
            //ECKey.ECDSASignature signature = new ECDSASignature();
            Boolean result = ECKey.verify(Hash.sha256(path.getBytes()), signature, ByteUtil.hexStringToByteArray(res.substring(0, 130)));
            if(!result) {
                throw new ImkeyException(Messages.IMKEY_PUBLIC_KEY_INVALID);
            }
        }
        */
        return res.substring(0, 130);
    }

    public String getCosmosXpubHex(String path, boolean verifyKey) {

        String getXpubApdu = Apdu.cosmosPub(path, verifyKey);
        String res = sendApdu(getXpubApdu);
        Apdu.checkResponse(res);

        // 65字节pub+32字节chaincode+签名值
        String signature = res.substring(97*2, res.length()-4);
        String data = res.substring(0, 97*2);

        Boolean verifyRes = Wallet.signVerify(ByteUtil.hexStringToByteArray(data), ByteUtil.hexStringToByteArray(signature));
        if(!verifyRes) {
            throw new ImkeyException(Messages.IMKEY_SIGNATURE_VERIFY_FAIL);
        }
        return res.substring(0, 130);
    }

    protected int getDepth(String path) {
        if (path.endsWith("/")) {
            path = path.substring(0,path.length()-1);
        }
        String[] pathArray = path.split("/");
        return pathArray.length - 1;
    }

    protected String[] getChildNumberPath(String path) {
        if (path.endsWith("/")) {
            path = path.substring(0,path.length()-1);
        }
        return path.split("/");
    }


    protected String getParentPath(String path) {
        if (path.endsWith("/")) {
            path = path.substring(0,path.length()-1);
        }

        return path.substring(0, path.lastIndexOf("/"));
    }


    protected String pub2Address(int version, String pubKey) {
        byte[] pubKeyHash = Utils.sha256hash160(NumericUtil.hexToBytes(pubKey));
        return toBase58(version, pubKeyHash);

    }

    public String calcSegWitAddress(int version, String pubKey) {

        byte[] pubKeyHash = Utils.sha256hash160(NumericUtil.hexToBytes(pubKey));
        String redeemScript = String.format("0x0014%s", NumericUtil.bytesToHex(pubKeyHash));
        byte[] redeemScriptHash = Utils.sha256hash160(NumericUtil.hexToBytes(redeemScript));
        byte[] addressBytes = new byte[1 + redeemScriptHash.length + 4];
        addressBytes[0] = (byte) version;
        System.arraycopy(redeemScriptHash, 0, addressBytes, 1, redeemScriptHash.length);
        byte[] checksum = Sha256Hash.hashTwice(addressBytes, 0, redeemScriptHash.length + 1);
        System.arraycopy(checksum, 0, addressBytes, redeemScriptHash.length + 1, 4);
        return Base58.encode(addressBytes);
    }

    public static String publicKeyToAddress(byte[] pubKeyBytes) {
        byte[] hashedBytes = Hash.keccak256(pubKeyBytes);
        byte[] addrBytes = Arrays.copyOfRange(hashedBytes, hashedBytes.length - 20, hashedBytes.length);
        return NumericUtil.bytesToHex(addrBytes);
    }

    protected String toBase58(int version, byte[] bytes ) {
        byte[] addressBytes = new byte[1 + bytes.length + 4];
        addressBytes[0] = (byte) version;
        System.arraycopy(bytes, 0, addressBytes, 1, bytes.length);
        byte[] checksum = Sha256Hash.hashTwice(addressBytes, 0, bytes.length + 1);
        System.arraycopy(checksum, 0, addressBytes, bytes.length + 1, 4);
        LogUtil.d(ByteUtil.byteArrayToHexString(addressBytes));
        return Base58.encode(addressBytes);
    }

    /**
     * Calculate the compressed format public key based on the uncompressed format public key
     * @param uncomprsPub
     * @return
     */
    protected String calComprsPub(String uncomprsPub) {
        String x = uncomprsPub.substring(2, 66);
        String y = uncomprsPub.substring(66,130);
        // if y is even
        BigInteger b = new BigInteger(y,16);
        String prefix = "02";
        if(!b.mod(BigInteger.valueOf(2)).equals(BigInteger.ZERO)) {
            prefix = "03";
        }
        return prefix + x;
    }

    /**
     * get fingerprint
     * @param pubKey  compressed public key
     * @return
     */
    protected int getFingerprint(byte[] pubKey) {
        return ByteBuffer.wrap(Arrays.copyOfRange(Utils.sha256hash160(pubKey), 0, 4)).getInt();
    }

    protected byte[] addChecksum(byte[] input) {
        int inputLength = input.length;
        byte[] checksummed = new byte[inputLength + 4];
        System.arraycopy(input, 0, checksummed, 0, inputLength);
        byte[] checksum = Sha256Hash.hashTwice(input);
        System.arraycopy(checksum, 0, checksummed, inputLength, 4);
        return checksummed;
    }

    protected List<ChildNumber> generatePath(String path) {
        List<ChildNumber> list = new ArrayList<>();
        for (String p : path.split("/")) {
            if ("m".equalsIgnoreCase(p) || "".equals(p.trim())) {
                continue;
            } else if (p.charAt(p.length() - 1) == '\'') {
                list.add(new ChildNumber(Integer.parseInt(p.substring(0, p.length() - 1)), true));
            } else {
                list.add(new ChildNumber(Integer.parseInt(p), false));
            }
        }
        return list;
    }

    public static String checkedEthAddress(String address) {
        final String cleanAddress = address.toLowerCase().replace("0x","");

        StringBuilder o = new StringBuilder();
        String keccakHex = ByteUtil.byteArrayToHexString(Hash.keccak256(address.getBytes()));

        char[] cs = cleanAddress.toLowerCase().toCharArray();
        for (int i = 0; i < cs.length; i++) {
            char c = cs[i];
            c = (Character.digit(keccakHex.charAt(i), 16) & 0xFF) > 7 ? Character.toUpperCase(c) : Character.toLowerCase(c);
            o.append(c);
        }
        return "0x" + o.toString();
    }

    public static byte[] signPackage(Sha256Hash hashData) {

        byte[] privateKey = KeyManager.getInstance().getPrivKey();
        LogUtil.d("privateKey:" + ByteUtil.byteArrayToHexString(privateKey));

        if(Arrays.equals(privateKey, new byte[32])) {
            throw new ImkeyException(Messages.IMKEY_NOT_BIND_CHECK);
        }

        ECKey ecKey = ECKey.fromPrivate(privateKey);
        ECKey.ECDSASignature ecSig = ecKey.sign(hashData);
        byte[] sig = ecSig.encodeToDER();
        return sig;

    }

    public static boolean signVerify(byte[] data, byte[] signature) {
        byte[] publicKey = KeyManager.getInstance().getSePubKey();
        if(Arrays.equals(publicKey, new byte[65])) {
            throw new ImkeyException(Messages.IMKEY_NOT_BIND_CHECK);
        }

        ECKey ecKey = ECKey.fromPublicOnly(publicKey);
        data = Hash.sha256(data);
        boolean res = ecKey.verify(data, signature);

        return res;
    }
}
