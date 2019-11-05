package com.mk.imkeylibrary.core.wallet.transaction;

import com.google.common.base.Strings;

import java.nio.charset.Charset;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.Wallet;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class ImKeyEOSTransaction extends Wallet implements TransactionSigner {

  private byte[] txBuf;
  private List<ToSignObj> txsToSign;

  public ImKeyEOSTransaction(byte[] txBuf) {
    this.txBuf = txBuf;
  }

  public ImKeyEOSTransaction(List<ToSignObj> txsToSign) {
    this.txsToSign = txsToSign;
  }

  @Deprecated
  public TransactionSignedResult signTransaction(String chainId, String path) {
    return null;
  }

  public List<TxMultiSignResult> signTransactions(String chainId, String to, String from, String payment, String path) {
    // path校验
    Path.checkPath(path);

    // 选择应用
    selectApplet();

    List<TxMultiSignResult> results = new ArrayList<>(txsToSign.size());
    for (ToSignObj toSignObj : txsToSign) {

      byte[] txBuf = NumericUtil.hexToBytes(toSignObj.txHex);
      String transactionID = NumericUtil.bytesToHex(Hash.sha256(txBuf));

      byte[] txChainIDBuf = ByteUtil.concat(NumericUtil.hexToBytes(chainId), txBuf);

      byte[] zeroBuf = new byte[32];
      Arrays.fill(zeroBuf, (byte) 0);
      byte[] fullTxBuf = ByteUtil.concat(txChainIDBuf, zeroBuf);

      byte[] hashedTx = Hash.sha256(fullTxBuf);
      byte[] viewInfo;
      if (Strings.isNullOrEmpty(payment)) {
        byte[] paymentPrefix = {0x07, (byte) 0x00};
        byte[] toPrefix = {0x08, (byte) 0x00};
        viewInfo = ByteUtil.concat(paymentPrefix, toPrefix);
      } else {
        byte[] paymentBytes = payment.getBytes(Charset.forName("UTF-8"));
        byte[] paymentPrefix = {0x07, (byte) paymentBytes.length};
        byte[] toBytes = to.getBytes(Charset.forName("UTF-8"));
        byte[] toPrefix = {0x08, (byte) toBytes.length};
        viewInfo = ByteUtil.concat(ByteUtil.concat(paymentPrefix, paymentBytes), ByteUtil.concat(toPrefix, toBytes));
      }

      List<String> signatures = new ArrayList<>(toSignObj.publicKeys.size());
      for (int i = 0; i < toSignObj.publicKeys.size(); i++) {
        String signed;
        String pubKey = toSignObj.publicKeys.get(i);
        byte[] pathBytes = path.getBytes();
        byte[] hashedTxPrefix = {0x01,0x20};  //TL
        byte[] pathPrefix = {0x02,(byte) pathBytes.length};  //TL;
        //byte[] txPack = ByteUtil.concat(viewInfo,ByteUtil.concat(ByteUtil.concat(hashedTxPrefix, hashedTx), ByteUtil.concat(pathPrefix, pathBytes)));
        byte[] txPack = ByteUtil.concat(ByteUtil.concat(hashedTxPrefix, hashedTx), ByteUtil.concat(ByteUtil.concat(pathPrefix, pathBytes), viewInfo));
        signed = EOSSign.sign(txPack, pubKey, hashedTx);
        signatures.add(signed);
      }

      TxMultiSignResult signedResult = new TxMultiSignResult(transactionID, signatures);
      results.add(signedResult);
    }
    return results;
  }

  public static class ToSignObj {
    private String txHex;
    private List<String> publicKeys;
    //private List<String> permissions; //@XM@20180901 need permission for imkey to get correct sign

    public String getTxHex() {
      return txHex;
    }

    public void setTxHex(String txHex) {
      this.txHex = txHex;
    }

    public List<String> getPublicKeys() {
      return publicKeys;
    }

    public void setPublicKeys(List<String> publicKeys) {
      this.publicKeys = publicKeys;
    }
  }


  @Override
  protected String getAid() {
    return Applet.EOS_AID;
  }

}
