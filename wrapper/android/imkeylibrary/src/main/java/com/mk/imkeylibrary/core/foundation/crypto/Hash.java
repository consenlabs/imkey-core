package com.mk.imkeylibrary.core.foundation.crypto;

import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

import org.bitcoinj.core.Sha256Hash;

import java.security.InvalidKeyException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

public class Hash {

  public static String keccak256(String hex) {
    byte[] bytes = NumericUtil.hexToBytes(hex);
    byte[] result = keccak256(bytes);
    return NumericUtil.bytesToHex(result);
  }

  public static byte[] keccak256(byte[] input) {
    return keccak256(input, 0, input.length);
  }

  public static byte[] generateMac(byte[] derivedKey, byte[] cipherText) {
    byte[] result = new byte[16 + cipherText.length];

    System.arraycopy(derivedKey, 16, result, 0, 16);
    System.arraycopy(cipherText, 0, result, 16, cipherText.length);

    return Hash.keccak256(result);
  }

  public static String sha256(String hexInput) {
    byte[] bytes = NumericUtil.hexToBytes(hexInput);
    byte[] result = sha256(bytes);
    return NumericUtil.bytesToHex(result);
  }

  public static byte[] sha256(byte[] input) {
    return sha256(input, 0, input.length);
  }

  private static byte[] keccak256(byte[] input, int offset, int length) {

    Keccak keccak = new Keccak(256);
    keccak.update(input, offset, length);

    return keccak.digest().array();
  }

  private static byte[] sha256(byte[] input, int offset, int length) {
    try {
      MessageDigest md = MessageDigest.getInstance("SHA-256");
      md.update(input, offset, length);
      return md.digest();
    } catch (Exception ex) {
      throw new ImkeyException(Messages.IMKEY_SHA_EXCEPTION);
    }
  }

  public static byte[] sha1(byte[] input) {
    return sha1(input, 0, input.length);
  }

  private static byte[] sha1(byte[] input, int offset, int length) {
    try {
      MessageDigest md = MessageDigest.getInstance("SHA");
      md.update(input, offset, length);
      return md.digest();
    } catch (Exception ex) {
      throw new ImkeyException(Messages.IMKEY_SHA_EXCEPTION);
    }
  }
}
