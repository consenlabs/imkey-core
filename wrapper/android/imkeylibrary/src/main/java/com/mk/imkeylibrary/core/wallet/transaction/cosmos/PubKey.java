package com.mk.imkeylibrary.core.wallet.transaction.cosmos;

public class PubKey {
  String type = "tendermint/PubKeySecp256k1";

  String value;

  public PubKey(String value) {
    this.value = value;
  }

  public String getType() {
    return type;
  }

  public void setType(String type) {
    this.type = type;
  }

  public String getValue() {
    return value;
  }

  public void setValue(String value) {
    this.value = value;
  }
}
