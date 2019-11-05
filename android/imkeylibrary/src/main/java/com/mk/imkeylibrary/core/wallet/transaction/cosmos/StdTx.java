package com.mk.imkeylibrary.core.wallet.transaction.cosmos;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;
import java.util.Map;

public class StdTx {
  @JsonProperty("msg")
  List<Map<String, Object>> msgs;
  StdFee fee;
  List<StdSignature> signatures;
  String memo;

  public StdTx(List<Map<String, Object>> msgs, StdFee fee, List<StdSignature> signatures, String memo) {
    this.msgs = msgs;
    this.fee = fee;
    this.signatures = signatures;
    this.memo = memo;
  }

  public List<Map<String, Object>> getMsgs() {
    return msgs;
  }

  public void setMsgs(List<Map<String, Object>> msgs) {
    this.msgs = msgs;
  }

  public StdFee getFee() {
    return fee;
  }

  public void setFee(StdFee fee) {
    this.fee = fee;
  }

  public List<StdSignature> getSignatures() {
    return signatures;
  }

  public void setSignatures(List<StdSignature> signatures) {
    this.signatures = signatures;
  }

  public String getMemo() {
    return memo;
  }

  public void setMemo(String memo) {
    this.memo = memo;
  }
}
