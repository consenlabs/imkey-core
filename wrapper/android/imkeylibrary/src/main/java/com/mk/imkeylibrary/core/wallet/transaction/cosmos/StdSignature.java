package com.mk.imkeylibrary.core.wallet.transaction.cosmos;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;

public class StdSignature {
  long accountNumber;
  String signature;
  PubKey pubKey;
  long sequence;

  @JsonSerialize(using=ToStringSerializer.class)
  public long getAccountNumber() {
    return accountNumber;
  }

  public void setAccountNumber(long accountNumber) {
    this.accountNumber = accountNumber;
  }

  @JsonSerialize(using= ToStringSerializer.class)
  public long getSequence() {
    return sequence;
  }

  public void setSequence(long sequence) {
    this.sequence = sequence;
  }

  public StdSignature(long accountNumber, String signature, PubKey pubKey, long sequence) {
    this.accountNumber = accountNumber;
    this.signature = signature;
    this.pubKey = pubKey;
    this.sequence = sequence;
  }

  public String getSignature() {
    return signature;
  }

  public void setSignature(String signature) {
    this.signature = signature;
  }

  public PubKey getPubKey() {
    return pubKey;
  }

  public void setPubKey(PubKey pubKey) {
    this.pubKey = pubKey;
  }
}
