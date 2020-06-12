package com.mk.imkeylibrary.core.wallet.transaction;

import org.bitcoinj.core.AddressFormatException;
import org.bitcoinj.core.Base58;
import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.VersionedChecksummedBytes;
import org.spongycastle.crypto.digests.RIPEMD160Digest;

import java.util.Arrays;

import com.mk.imkeylibrary.utils.ByteUtil;

public class EOSKey extends VersionedChecksummedBytes {

  protected EOSKey(String encoded) throws AddressFormatException {
    super(encoded);
  }

  protected EOSKey(int version, byte[] bytes) {
    super(version, bytes);
  }

  public static EOSKey fromWIF(String wif) {
    return new EOSKey(wif);
  }

  public static EOSKey fromPrivate(byte[] prvKey) {
    // EOS doesn't distinguish between mainnet and testnet.
    return new EOSKey(128, prvKey);
  }

  public static String privateToPublicKey(byte[] prvKey) {
    return new EOSKey(128, prvKey).getPublicKeyAsHex();
  }

  public String getPublicKeyAsHex() {
    ECKey ecKey = ECKey.fromPrivate(bytes);
    byte[] pubKeyData = ecKey.getPubKey();
    RIPEMD160Digest digest = new RIPEMD160Digest();
    digest.update(pubKeyData, 0, pubKeyData.length);
    byte[] out = new byte[20];
    digest.doFinal(out, 0);
    byte[] checksumBytes = Arrays.copyOfRange(out, 0, 4);

    pubKeyData = ByteUtil.concat(pubKeyData, checksumBytes);
    return "EOS" + Base58.encode(pubKeyData);
  }
  //@XM@20180903 add this to get pubKey from Apdu result
  public static String getPublicKeyApdu(byte[] pubKeyData) {
    RIPEMD160Digest digest = new RIPEMD160Digest();
    digest.update(pubKeyData, 0, pubKeyData.length);
    byte[] out = new byte[20];
    digest.doFinal(out, 0);
    byte[] checksumBytes = Arrays.copyOfRange(out, 0, 4);

    pubKeyData = ByteUtil.concat(pubKeyData, checksumBytes);
    return "EOS" + Base58.encode(pubKeyData);
  }

  ECKey getECKey() {
    return ECKey.fromPrivate(bytes, true);
  }

}
