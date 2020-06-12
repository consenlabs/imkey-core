package com.mk.imkeylibrary.core.wallet.transaction.cosmos;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;

public class Coin {
  long amount;
  String denom;


  public Coin(String denom, long amount) {
    this.denom = denom;
    this.amount = amount;
  }

  @JsonSerialize(using= ToStringSerializer.class)
  public long getAmount() {
    return amount;
  }

  public void setAmount(long amount) {
    this.amount = amount;
  }

  public String getDenom() {
    return denom;
  }

  public void setDenom(String denom) {
    this.denom = denom;
  }


}
