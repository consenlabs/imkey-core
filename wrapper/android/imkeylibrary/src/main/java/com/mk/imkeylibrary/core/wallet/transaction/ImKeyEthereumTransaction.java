package com.mk.imkeylibrary.core.wallet.transaction;


import java.math.BigInteger;
import java.nio.charset.Charset;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.foundation.rlp.RlpEncoder;
import com.mk.imkeylibrary.core.foundation.rlp.RlpList;
import com.mk.imkeylibrary.core.foundation.rlp.RlpString;
import com.mk.imkeylibrary.core.foundation.rlp.RlpType;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.Wallet;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

/**
 * Transaction class used for signing transactions locally.<br>
 * For the specification, refer to p4 of the <a href="http://gavwood.com/paper.pdf">
 * <p>
 * yellow paper</a>.
 */
public class ImKeyEthereumTransaction extends Wallet implements TransactionSigner {

  private BigInteger nonce;
  private BigInteger gasPrice;
  private BigInteger gasLimit;
  private String to;
  private BigInteger value;
  private String data;
  private String payment;
  private String receiver;
  private String sender;
  private String fee;

  public ImKeyEthereumTransaction(BigInteger nonce, BigInteger gasPrice, BigInteger gasLimit, String to,
                                  BigInteger value, String data, HashMap<String, String> preview) {
    this.nonce = nonce;
    this.gasPrice = gasPrice;
    this.gasLimit = gasLimit;
    this.to = to;
    this.value = value;

    if (data != null) {
      this.data = NumericUtil.cleanHexPrefix(data);
    }

    if (preview.isEmpty()) {
        throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
    } else {
        this.payment = preview.get("payment");
        this.receiver = preview.get("receiver");
        this.sender = preview.get("sender");
        this.fee = preview.get("fee");
    }
  }

  public BigInteger getNonce() {
    return nonce;
  }

  public BigInteger getGasPrice() {
    return gasPrice;
  }

  public BigInteger getGasLimit() {
    return gasLimit;
  }

  public String getTo() {
    return to;
  }

  public BigInteger getValue() {
    return value;
  }

  public String getData() {
    return data;
  }

  public String getPayment() {
    return payment;
  }

  public String getReceiver() {
    return receiver;
  }

  public String getSender() {
      return sender;
  }

  public String getFee() {
      return fee;
  }

  @Override
  public TransactionSignedResult signTransaction(String chainID, String path) {

    // path校验
    Path.checkPath(path);
    // 选择应用
    selectApplet();

    String signedTx = signTransaction((byte) Integer.parseInt(chainID), path.getBytes());
    String txHash = this.calcTxHash(signedTx);
    return new TransactionSignedResult(signedTx, txHash);
  }

  String signTransaction(byte chainId, byte[] path) {

    SignatureData signatureData = new SignatureData(chainId, new byte[]{}, new byte[]{});
    byte[] encodedTx = encodeToRLP(signatureData);
    //byte[] encodedTxWtl = ByteUtil.concat(new byte[]{(byte)0x01}, ByteUtil.concat(new byte[]{(byte)encodedTx.length},encodedTx));
    // preview: payment + to(receiver) + fee in TLV format
    byte[] payArray = getPayment().getBytes(Charset.forName("UTF-8"));
    byte[] payArrayWtl = ByteUtil.concat(new byte[]{(byte)0x07}, ByteUtil.concat(new byte[]{(byte)payArray.length},payArray));

    byte[] recArray = getReceiver().getBytes(Charset.forName("UTF-8"));
    byte[] recArrayWtl = ByteUtil.concat(new byte[]{(byte)0x08}, ByteUtil.concat(new byte[]{(byte)recArray.length},recArray));

    byte[] feeArray = getFee().getBytes(Charset.forName("UTF-8"));
    byte[] feeArrayWtl = ByteUtil.concat(new byte[]{(byte)0x09}, ByteUtil.concat(new byte[]{(byte)feeArray.length},feeArray));

    byte[] senderArray = getSender().getBytes(Charset.forName("UTF-8"));
    signatureData = EthereumSign.signMessage(encodedTx, path, payArrayWtl, recArrayWtl, feeArrayWtl, senderArray);

    SignatureData eip155SignatureData = createEip155SignatureData(signatureData, chainId);
    byte[] rawSignedTx = encodeToRLP(eip155SignatureData);
    return NumericUtil.bytesToHex(rawSignedTx);
  }

  String calcTxHash(String signedTx) {
    return NumericUtil.prependHexPrefix(Hash.keccak256(signedTx));
  }

  private static SignatureData createEip155SignatureData(SignatureData signatureData, byte chainId) {
    int v = signatureData.getV() + (chainId * 2) + 8;

    return new SignatureData(v, signatureData.getR(), signatureData.getS());
  }

  byte[] encodeToRLP(SignatureData signatureData) {
    List<RlpType> values = asRlpValues(signatureData);
    RlpList rlpList = new RlpList(values);
    return RlpEncoder.encode(rlpList);
  }

  List<RlpType> asRlpValues(SignatureData signatureData) {
    List<RlpType> result = new ArrayList<>();

    result.add(RlpString.create(getNonce()));
    result.add(RlpString.create(getGasPrice()));
    result.add(RlpString.create(getGasLimit()));

    // an empty to address (contract creation) should not be encoded as a numeric 0 value
    String to = getTo();
    if (to != null && to.length() > 0) {
      // addresses that start with zeros should be encoded with the zeros included, not
      // as numeric values
      result.add(RlpString.create(NumericUtil.hexToBytes(to)));
    } else {
      result.add(RlpString.create(""));
    }

    result.add(RlpString.create(getValue()));

    // value field will already be hex encoded, so we need to convert into binary first
    byte[] data = NumericUtil.hexToBytes(getData());
    result.add(RlpString.create(data));

    if (signatureData != null && (long)(signatureData.getV() & 0xffffffffl) > 0) {
      result.add(RlpString.create(signatureData.getV()));
      result.add(RlpString.create(ByteUtil.trimLeadingZeroes(signatureData.getR())));
      result.add(RlpString.create(ByteUtil.trimLeadingZeroes(signatureData.getS())));
    }

    return result;
  }

  @Override
  protected String getAid() {
    return Applet.ETH_AID;
  }

}