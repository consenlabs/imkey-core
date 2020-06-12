package com.mk.imkeylibrary.core.wallet.transaction;

import org.bitcoinj.core.Base58;
import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.Sha256Hash;
import org.spongycastle.crypto.digests.RIPEMD160Digest;
import org.spongycastle.crypto.digests.SHA256Digest;
import org.spongycastle.crypto.params.ECPrivateKeyParameters;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.wallet.Wallet;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

import static org.bitcoinj.core.ECKey.CURVE;

public class EOSSign {

  @Deprecated
  public static String sign(byte[] dataSha256, String wif) {
    SignatureData signatureData = signAsRecoverable(dataSha256, EOSKey.fromWIF(wif).getECKey());
    byte[] sigResult = ByteUtil.concat(NumericUtil.intToBytes(signatureData.getV()), signatureData.getR());
    sigResult = ByteUtil.concat(sigResult, signatureData.getS());
    return serialEOSSignature(sigResult);
  }

  public static String sign(byte[] dataSha256, byte[] prvKey) {
    ECKey ecKey = EOSKey.fromPrivate(prvKey).getECKey();
    SignatureData signatureData = signAsRecoverable(dataSha256, ecKey);
    byte[] sigResult = ByteUtil.concat(NumericUtil.intToBytes(signatureData.getV()), signatureData.getR());
    sigResult = ByteUtil.concat(sigResult, signatureData.getS());
    return serialEOSSignature(sigResult);
  }

  public static String sign(byte[] txPack, String pubKey, byte[] hashedTx) {

    byte[] hashData  = Sha256Hash.hashTwice(txPack);
    byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
    byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signature.length},signature));
    byte[] apduPack = ByteUtil.concat(signatureWtl, txPack);

    List<String> prepares = Apdu.eosPrepare(NumericUtil.bytesToHex(apduPack));

    String res = "";
    for (String apduPre : prepares) {
       res = Ble.getInstance().sendApdu(apduPre, Constants.SEND_SIGN_PRE_APDU_TIMEOUT);
       Apdu.checkResponse(res);
    }
    String eosPubkEy = EOSKey.getPublicKeyApdu(NumericUtil.hexToBytes(calComprsPub(res)));
    if (eosPubkEy.equals(pubKey)) {
      int nonce = 0;  //@XM@20180921 align with eosjs-ecc line 209@https://github.com/EOSIO/eosjs-ecc/blob/master/src/signature.js
      byte[] byteR, byteS;
      byte byteV;
      ECKey.ECDSASignature signatureData = null;
      while (true) {
        //send adpu-sign if compare result is ok：interate nonce value to sign the package
        String apduSign = Apdu.eosTxSign(nonce);
        String signRes = Ble.getInstance().sendApdu(apduSign);
        Apdu.checkResponse(signRes);
        String r = signRes.substring(2, 66);
        String s = signRes.substring(66, 130);
        byteR = NumericUtil.hexToBytes(r);
        byteS = NumericUtil.hexToBytes(s);
        BigInteger R = new BigInteger(byteR);
        BigInteger S = new BigInteger(byteS);

        signatureData = new ECKey.ECDSASignature(R, S).toCanonicalised();
        byte[] der = signatureData.encodeToDER();
        byteV = calV(hashedTx, signatureData, res);

        int lenR = der[3];
        int lenS = der[5 + lenR];
        if (lenR == 32 && lenS == 32) {
          break;
        }
        nonce++;
      }
      int headerByte = byteV;

      byte[] sigResult = ByteUtil.concat(new byte[]{(byte)headerByte}, byteR);  //@XM@20180904 seems missing +27 + 4..
      byteS = NumericUtil.bigIntegerToBytesWithZeroPadded(signatureData.s, 32);
      sigResult = ByteUtil.concat(sigResult, byteS);
      return serialEOSSignature(sigResult);
    } else {
      throw new ImkeyException(Messages.IMKEY_PUBLICKEY_MISMATCH_WITH_PATH);
    }

  }


  public static String signMessage(byte[] txPack, String pubKey, byte[] hashedTx) {
    selectApplet();
    ByteUtil.byteArrayToHexString(Sha256Hash.hash(txPack));
    byte[] hashData = Sha256Hash.hashTwice(txPack);
    byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
    byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte) 0x00}, ByteUtil.concat(new byte[]{(byte) signature.length}, signature));
    byte[] apduPack = ByteUtil.concat(signatureWtl, txPack);

    List<String> prepares = Apdu.eosMsgPrepare(NumericUtil.bytesToHex(apduPack));

    String res = "";
    for (String apduPre : prepares) {
      res = Ble.getInstance().sendApdu(apduPre, Constants.SEND_SIGN_PRE_APDU_TIMEOUT);
      Apdu.checkResponse(res);
    }
    String eosPubkEy = EOSKey.getPublicKeyApdu(NumericUtil.hexToBytes(calComprsPub(res)));
    if (eosPubkEy.equals(pubKey)) {
      int nonce = 0;  //@XM@20180921 align with eosjs-ecc line 209@https://github.com/EOSIO/eosjs-ecc/blob/master/src/signature.js
      byte[] byteR, byteS;
      byte byteV;
      ECKey.ECDSASignature signatureData = null;
      while (true) {
        //send adpu-sign if compare result is ok：interate nonce value to sign the package
        String apduSign = Apdu.eosMsgSign(nonce);
        String signRes = Ble.getInstance().sendApdu(apduSign);
        Apdu.checkResponse(signRes);
        String r = signRes.substring(2, 66);
        String s = signRes.substring(66, 130);
        byteR = NumericUtil.hexToBytes(r);
        byteS = NumericUtil.hexToBytes(s);
        BigInteger R = new BigInteger(byteR);
        BigInteger S = new BigInteger(byteS);

        signatureData = new ECKey.ECDSASignature(R, S).toCanonicalised();
        byte[] der = signatureData.encodeToDER();
        byteV = calV(hashedTx, signatureData, res);

        int lenR = der[3];
        int lenS = der[5 + lenR];
        if (lenR == 32 && lenS == 32) {
          break;
        }
        nonce++;
      }
      int headerByte = byteV;

      byte[] sigResult = ByteUtil.concat(new byte[]{(byte) headerByte}, byteR);  //@XM@20180904 seems missing +27 + 4..
      byteS = NumericUtil.bigIntegerToBytesWithZeroPadded(signatureData.s, 32);
      sigResult = ByteUtil.concat(sigResult, byteS);
      return serialEOSSignature(sigResult);
    } else {
      throw new ImkeyException(Messages.IMKEY_PUBLICKEY_MISMATCH_WITH_PATH);
    }
  }

    private static SignatureData signAsRecoverable(byte[] value, ECKey ecKey) {
    int recId = -1;
    ECKey.ECDSASignature sig = eosSign(value, ecKey.getPrivKey());
    for (int i = 0; i < 4; i++) {
      ECKey recoverKey = ECKey.recoverFromSignature(i, sig, Sha256Hash.wrap(value), false);
      if (recoverKey != null && recoverKey.getPubKeyPoint().equals(ecKey.getPubKeyPoint())) {
        recId = i;
        break;
      }
    }

    if (recId == -1) {
      throw new ImkeyException("Could not construct a recoverable key. This should never happen.");
    }
    int headerByte = recId + 27 + 4;
    // 1 header + 32 bytes for R + 32 bytes for S
    byte v = (byte) headerByte;
    byte[] r = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.r, 32);
    byte[] s = NumericUtil.bigIntegerToBytesWithZeroPadded(sig.s, 32);

    return new SignatureData(v, r, s);

  }

  private static ECKey.ECDSASignature eosSign(byte[] input, BigInteger privateKeyForSigning) {
    EOSECDSASigner signer = new EOSECDSASigner(new MyHMacDSAKCalculator(new SHA256Digest()));
    ECPrivateKeyParameters privKey = new ECPrivateKeyParameters(privateKeyForSigning, CURVE);
    signer.init(true, privKey);
    BigInteger[] components = signer.generateSignature(input);
    return new ECKey.ECDSASignature(components[0], components[1]).toCanonicalised();
  }

  private static String serialEOSSignature(byte[] data) {
    LogUtil.d("data:" + ByteUtil.byteArrayToHexString(data) );
    byte[] toHash = ByteUtil.concat(data, "K1".getBytes());
    RIPEMD160Digest digest = new RIPEMD160Digest();
    digest.update(toHash, 0, toHash.length);
    byte[] out = new byte[20];
    digest.doFinal(out, 0);
    byte[] checksumBytes = Arrays.copyOfRange(out, 0, 4);
    data = ByteUtil.concat(data, checksumBytes);
    return "SIG_K1_" + Base58.encode(data);
  }

  private static String selectApplet() {
    String selectApdu = Apdu.select(Applet.EOS_AID);
    return Ble.getInstance().sendApdu(selectApdu);
  }

  private static String calComprsPub(String uncomprsPub) {
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

  private static byte calV(byte[] txHash, ECKey.ECDSASignature sig, String pubKeyHex) {
    // 获取公钥
    ECKey ecKey = ECKey.fromPublicOnly(NumericUtil.hexToBytes(pubKeyHex.substring(0,130)));
    int recId = -1;
    for (int i = 0; i < 4; i++) {
      ECKey recoverKey = ECKey.recoverFromSignature(i, sig, Sha256Hash.wrap(txHash), false);
      if (recoverKey != null && recoverKey.getPubKeyPoint().equals(ecKey.getPubKeyPoint())) {
        recId = i;
        break;
      }
    }
    if (recId == -1) {
      throw new RuntimeException(
              "Could not construct a recoverable key. This should never happen.");
    }

    int headerByte = recId + 27 + 4;
    // 1 header + 32 bytes for R + 32 bytes for S
    byte v = (byte) headerByte;
    return v;
  }
}
