package com.mk.imkeylibrary.core.wallet.transaction.cosmos;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;

import java.util.List;

public class StdFee {
  List<Coin> amount;
  long gas;

  public StdFee(List<Coin> amount, long gas) {
    this.amount = amount;
    this.gas = gas;
  }

  public List<Coin> getAmount() {
    return amount;
  }

  public void setAmount(List<Coin> amount) {
    this.amount = amount;
  }

  @JsonSerialize(using= ToStringSerializer.class)
  public long getGas() {
    return gas;
  }

  public void setGas(long gas) {
    this.gas = gas;
  }
}
