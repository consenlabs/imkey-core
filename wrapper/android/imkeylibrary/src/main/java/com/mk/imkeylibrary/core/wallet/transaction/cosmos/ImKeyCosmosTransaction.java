package com.mk.imkeylibrary.core.wallet.transaction.cosmos;


import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;
import com.google.common.base.Strings;

import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.Sha256Hash;
import org.spongycastle.util.encoders.Base64;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.charset.Charset;
import java.util.Collections;
import java.util.List;
import java.util.Map;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.EccUtil;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.Wallet;
import com.mk.imkeylibrary.core.wallet.transaction.TransactionSignedResult;
import com.mk.imkeylibrary.core.wallet.transaction.TransactionSigner;
import com.mk.imkeylibrary.core.wallet.transaction.TxMultiSignResult;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;


public class ImKeyCosmosTransaction implements TransactionSigner {

  long accountNumber;
  String chainId;
  StdFee fee;
  String memo;
  List<Map<String, Object>> msgs;
  long sequence;

  @JsonSerialize(using= ToStringSerializer.class)
  public long getAccountNumber() {
    return accountNumber;
  }

  public void setAccountNumber(long accountNumber) {
    this.accountNumber = accountNumber;
  }

  public String getChainId() {
    return chainId;
  }

  public void setChainId(String chainId) {
    this.chainId = chainId;
  }

  public StdFee getFee() {
    return fee;
  }

  public void setFee(StdFee fee) {
    this.fee = fee;
  }

  public String getMemo() {
    return memo;
  }

  public void setMemo(String memo) {
    this.memo = memo;
  }

  public List<Map<String, Object>> getMsgs() {
    return msgs;
  }

  public void setMsgs(List<Map<String, Object>> msgs) {
    this.msgs = msgs;
  }

  @JsonSerialize(using=ToStringSerializer.class)
  public long getSequence() {
    return sequence;
  }

  public void setSequence(long sequence) {
    this.sequence = sequence;
  }

  public ImKeyCosmosTransaction(long accountNumber, String chainId, StdFee fee, String memo, List<Map<String, Object>> msgs, long sequence) {
    this.accountNumber = accountNumber;
    this.chainId = chainId;
    this.fee = fee;
    this.memo = memo;
    this.msgs = msgs;
    this.sequence = sequence;
  }

  private ObjectMapper jsonMapper() {
    ObjectMapper objectMapper = new ObjectMapper();
    objectMapper.configure(SerializationFeature.ORDER_MAP_ENTRIES_BY_KEYS, true);
    objectMapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
    objectMapper.setPropertyNamingStrategy(new PropertyNamingStrategy.SnakeCaseStrategy());
    return objectMapper;
  }

  private byte[] serializeTx() {
    try {
      ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
      jsonMapper().writeValue(outputStream, this);
      String data = new String(outputStream.toByteArray());
      LogUtil.d(data);
      return outputStream.toByteArray();
    } catch (IOException ex) {
      throw new ImkeyException(Messages.IMKEY_COSMOS_JSON_ERROR, ex);
    }
  }
  @Deprecated
  public TransactionSignedResult signTransaction(String chainId, String path) {
    return null;
  }
  public TransactionSignedResult signTransaction(String chainId, String path, String paymentDis, String toDis, String fromDis, String feeDis) {

    // path校验
    Path.checkPath(path);
    // 选择应用
    selectApplet();

    byte[] hashedTx = Hash.sha256(serializeTx());
    byte[] hashedTxPrefix = {0x01,0x20};  //TL

    byte[] viewInfo;
    if(Strings.isNullOrEmpty(paymentDis)) {
      byte[] paymentPrefix = {0x07, 0x00};
      byte[] toPrefix = {0x08, 0x00};
      byte[] feePrefix = {0x09, 0x00};
      viewInfo = ByteUtil.concat(paymentPrefix, toPrefix);
      viewInfo = ByteUtil.concat(viewInfo, feePrefix);
    } else {
      byte[] paymentBytes = paymentDis.getBytes(Charset.forName("UTF-8"));
      byte[] paymentPrefix = {0x07, (byte) paymentBytes.length};
      byte[] toBytes = toDis.getBytes(Charset.forName("UTF-8"));
      byte[] toPrefix = {0x08, (byte) toBytes.length};
      byte[] feeBytes = feeDis.getBytes(Charset.forName("UTF-8"));
      byte[] feePrefix = {0x09, (byte) feeBytes.length};
      viewInfo = ByteUtil.concat(ByteUtil.concat(paymentPrefix, paymentBytes), ByteUtil.concat(toPrefix, toBytes));
      viewInfo = ByteUtil.concat(viewInfo, ByteUtil.concat(feePrefix, feeBytes));
    }

    byte[] txPack = ByteUtil.concat(ByteUtil.concat(hashedTxPrefix, hashedTx), viewInfo);

    //通信数据签名
    byte[] hashData  = Sha256Hash.hashTwice(txPack);
    byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
    byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signature.length},signature));
    byte[] apduPack = ByteUtil.concat(signatureWtl, txPack);

    List<String> pres = Apdu.cosmosPrepare(NumericUtil.bytesToHex(apduPack));

    String res = "";
    for (int i = 0; i < pres.size(); i++) {
        String apdu = pres.get(i);
        int timeout = Constants.SENT_APDU_TIMEOUT;
        if (i == (pres.size()-1)) {
            timeout = Constants.SEND_SIGN_PRE_APDU_TIMEOUT;
        }
        res = Ble.getInstance().sendApdu(apdu, timeout);
        Apdu.checkResponse(res);
    }

    // 签名
    String apdu = Apdu.cosmosSign(path);
    String signRes = Ble.getInstance().sendApdu(apdu);
    Apdu.checkResponse(signRes);

    String rHex = signRes.substring(2, 66);
    String sHex = signRes.substring(66, 130);

    byte[] r = NumericUtil.bigIntegerToBytesWithZeroPadded(new BigInteger(rHex, 16), 32);
    byte[] s = NumericUtil.bigIntegerToBytesWithZeroPadded(EccUtil.getLowS(new BigInteger(sHex, 16)), 32);
    byte[] signatureBytes = ByteUtil.concat(r,s);
    String signatureStr = Base64.toBase64String(signatureBytes);

    //获取公钥
    String pubKey = getCosmosXpubHex(path, true);
    ECKey ecKey = EccUtil.getCompressECKey(NumericUtil.hexToBytes(pubKey));
    String publicKey = Base64.toBase64String(ecKey.getPubKey());

    StdSignature stdSignature = new StdSignature(accountNumber, signatureStr, new PubKey(publicKey), sequence);
    StdTx stdTx = new StdTx(msgs, fee, Collections.singletonList(stdSignature), memo);
    TransactionSignedResult signResult;
    try {
      ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
      jsonMapper().writeValue(outputStream, stdTx);
      String signedTx = new String(outputStream.toByteArray());
      signResult = new TransactionSignedResult(signedTx, "");
    } catch (IOException ex) {
      throw new ImkeyException(Messages.IMKEY_COSMOS_JSON_ERROR, ex);
    }
    return signResult;

  }

  public String getCosmosXpubHex(String path, boolean verifyKey) {

    String getXpubApdu = Apdu.cosmosPub(path, verifyKey);
    String res = Ble.getInstance().sendApdu(getXpubApdu);
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


  protected void selectApplet() {
    String selectApdu = Apdu.select(Applet.COSMOS_AID);
    String res = Ble.getInstance().sendApdu(selectApdu);
    Apdu.checkResponse(res);
  }

}

