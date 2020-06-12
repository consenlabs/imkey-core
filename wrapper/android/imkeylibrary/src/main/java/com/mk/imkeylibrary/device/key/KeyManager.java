package com.mk.imkeylibrary.device.key;

import com.google.common.io.BaseEncoding;

import org.bitcoinj.core.ECKey;

import com.mk.imkeylibrary.core.foundation.crypto.AES;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;

public class KeyManager {

    private byte[] privKey = new byte[32];
    private byte[] pubKey = new byte[65];
    private byte[] sePubKey = new byte[65];
    private byte[] sessionKey = new byte[16];
    private byte[] checksum = new byte[4];

    private byte[] encryKey = new byte[16];;
    private byte[] IV = new byte[16];

    private static KeyManager instance;

    private KeyManager(){

    }

    public static KeyManager getInstance() {
        if(instance == null){
            instance = new KeyManager();
        }
        return instance;
    }

    public Boolean decryptKeys(String ciphertext) {

        // base64解码
        byte[] cipherKeysByte = BaseEncoding.base64().decode(ciphertext);
        // 解密
        byte[] keysByte = AES.decryptByCBC(cipherKeysByte, encryKey, IV);

        // 解密失败
        if(keysByte.length==0) {
            return false;
        }

        System.arraycopy(keysByte, 0, privKey, 0, 32);
        System.arraycopy(keysByte, 32, pubKey, 0, 65);
        System.arraycopy(keysByte, 97, sePubKey, 0, 65);
        System.arraycopy(keysByte, 162, sessionKey, 0, 16);
        System.arraycopy(keysByte, 178, checksum, 0, 4);

        // 验证checksum
        byte[] data = new byte[178];
        System.arraycopy(keysByte, 0, data, 0, 178);
        byte[] hash = Hash.sha256(data);

        for(int i=0; i<checksum.length; i++) {
            if(checksum[i] != hash[i]) {
                return false;
            }
        }
        return true;
    }

    public String encryptKeys() {
        byte[] data = new byte[178];

        System.arraycopy(privKey, 0, data, 0, 32);
        System.arraycopy(pubKey, 0, data, 32, 65);
        System.arraycopy(sePubKey, 0, data, 97, 65);
        System.arraycopy(sessionKey, 0, data, 162, 16);
        byte[] hash = Hash.sha256(data);

        byte[] keysByte = new byte[182];
        System.arraycopy(data, 0, keysByte, 0, 178);
        System.arraycopy(hash, 0, keysByte, 178, 4);

        // 加密
        byte[] cipherKeys = AES.encryptByCBC(keysByte, encryKey, IV);

        return BaseEncoding.base64().encode(cipherKeys);
    }


    public void genEncryKey(String seid, String sn) {

        byte[] seidHash = Hash.sha256(seid.getBytes());
        byte[] snHash = Hash.sha256(sn.getBytes());
        for(int i=0; i<seidHash.length; i++) {
            seidHash[i] = (byte)(seidHash[i] ^ snHash[i]);
        }

        // 取前16个字节作为加密密钥
        System.arraycopy(seidHash, 0, encryKey, 0, 16);
        // 取后16字节作为IV值
        System.arraycopy(seidHash, seidHash.length-16, IV, 0, 16);
    }

    public void genLocalKeys() {
        ECKey ecKey = new ECKey();
        privKey = ecKey.getPrivKeyBytes();
        pubKey = ecKey.decompress().getPubKey();
    }

    public byte[] getPrivKey() {
        return privKey;
    }

    public void setPrivKey(byte[] privKey) {
        this.privKey = privKey;
    }

    public byte[] getPubKey() {
        return pubKey;
    }

    public void setPubKey(byte[] pubKey) {
        this.pubKey = pubKey;
    }

    public byte[] getSePubKey() {
        return sePubKey;
    }

    public void setSePubKey(byte[] sePubKey) {
        this.sePubKey = sePubKey;
    }


    public byte[] getChecksum() {
        return checksum;
    }

    public void setChecksum(byte[] checksum) {
        this.checksum = checksum;
    }

    public byte[] getEncryKey() {
        return encryKey;
    }

    public void setEncryKey(byte[] encryKey) {
        this.encryKey = encryKey;
    }

    public byte[] getSessionKey() {
        return sessionKey;
    }

    public void setSessionKey(byte[] sessionKey) {
        this.sessionKey = sessionKey;
    }
}
