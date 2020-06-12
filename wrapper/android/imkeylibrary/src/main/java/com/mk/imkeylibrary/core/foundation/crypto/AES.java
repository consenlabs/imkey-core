package com.mk.imkeylibrary.core.foundation.crypto;

import javax.crypto.Cipher;
import javax.crypto.spec.IvParameterSpec;
import javax.crypto.spec.SecretKeySpec;

import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;

public class AES {

  public enum AESType {
    CTR, CBC
  }
  
  public static byte[] encryptByCBC(byte[] data, byte[] key, byte[] iv) {
    return doAES(data, key, iv, Cipher.ENCRYPT_MODE, AESType.CBC, "PKCS5Padding");
  }

  public static byte[] decryptByCBC(byte[] ciphertext, byte[] key, byte[] iv) {
    return doAES(ciphertext, key, iv, Cipher.DECRYPT_MODE, AESType.CBC, "PKCS5Padding");
  }

  private static byte[] doAES(byte[] data, byte[] key, byte[] iv, int cipherMode, AESType type, String paddingType) {
    String aesType;
    if (type == AESType.CBC) {
      aesType = "CBC";
    } else {
      aesType = "CTR";
    }
    try {
      IvParameterSpec ivParameterSpec = new IvParameterSpec(iv);
      SecretKeySpec secretKeySpec = new SecretKeySpec(key, "AES");

      String algorithmDesc = String.format("AES/%s/%s", aesType, paddingType);
      Cipher cipher = Cipher.getInstance(algorithmDesc);
      cipher.init(cipherMode, secretKeySpec, ivParameterSpec);
      return cipher.doFinal(data);
    } catch (Throwable e) {
        throw new ImkeyException(Messages.IMKEY_AES_EXCEPTION);
    }
  }

}
