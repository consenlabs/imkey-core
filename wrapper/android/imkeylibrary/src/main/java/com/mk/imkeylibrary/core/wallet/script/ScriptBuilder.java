package com.mk.imkeylibrary.core.wallet.script;

import org.bitcoinj.crypto.TransactionSignature;
import org.bitcoinj.script.Script;

import javax.annotation.Nullable;

public class ScriptBuilder {

    /**
     * Creates a scriptSig that can redeem a pay-to-address output.
     * If given signature is null, incomplete scriptSig will be created with OP_0 instead of signature
     */
    public static Script createInputScript(@Nullable TransactionSignature signature, byte[] pubkeyBytes) {
        byte[] sigBytes = signature != null ? signature.encodeToBitcoin() : new byte[]{};
        return new org.bitcoinj.script.ScriptBuilder().data(sigBytes).data(pubkeyBytes).build();
    }

}
