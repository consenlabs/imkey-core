package com.mk.imkeylibrary.core.wallet.transaction;


public interface TransactionSigner {
  TransactionSignedResult signTransaction(String chainId, String path) ;


}
