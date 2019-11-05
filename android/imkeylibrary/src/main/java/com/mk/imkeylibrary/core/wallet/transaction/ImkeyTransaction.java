package com.mk.imkeylibrary.core.wallet.transaction;


import org.bitcoinj.core.NetworkParameters;
import org.bitcoinj.core.Transaction;
import org.bitcoinj.core.TransactionInput;
import org.bitcoinj.core.TransactionOutput;
import org.bitcoinj.core.UnsafeByteArrayOutputStream;
import org.bitcoinj.core.Utils;
import org.bitcoinj.crypto.TransactionSignature;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.math.BigInteger;

import static org.bitcoinj.core.Utils.uint32ToByteStreamLE;

public class ImkeyTransaction extends Transaction {


    public ImkeyTransaction(NetworkParameters params) {
        super(params);
    }

    public byte[] serializeTransaction(SigHash type, boolean anyoneCanPay) {

        try {
            int sigHash = TransactionSignature.calcSigHashValue(type, anyoneCanPay);
            byte[] data = this.bitcoinSerialize();

            ByteArrayOutputStream bos = new UnsafeByteArrayOutputStream(data.length + 4);
            bos.write(data, 0, data.length);
            // We also have to write a hash type (sigHashType is actually an unsigned char)
            uint32ToByteStreamLE(0x000000ff & (byte) sigHash, bos);
            return bos.toByteArray();
        } catch (IOException e) {
            throw new RuntimeException(e);  // Cannot happen.
        }
    }

    public byte[] serializeSegWitTransaction(SigHash type, boolean anyoneCanPay, int insize, int outsize, long[] inputvalue) {

        try {
            UnsafeByteArrayOutputStream stream = new UnsafeByteArrayOutputStream();
            Utils.uint32ToByteStreamLE(2L, stream);
            stream.write(insize);
            for(int i = 0; i < insize; i++) {
                TransactionInput input = this.getInput(i);
                input.bitcoinSerialize(stream);
                Utils.uint64ToByteStreamLE(BigInteger.valueOf(inputvalue[i]), stream);
            }
            stream.write(outsize);
            for(int i = 0; i < outsize; i++) {
                TransactionOutput output = this.getOutput(i);
                output.bitcoinSerialize(stream);
            }
            // write locktime
            Utils.uint32ToByteStreamLE(0x00000000, stream);
            // write sign type
            int sigHash = TransactionSignature.calcSigHashValue(type, anyoneCanPay);
            Utils.uint32ToByteStreamLE(0x000000ff & (byte) sigHash, stream);

            return stream.toByteArray();
        } catch (IOException e) {
            throw new RuntimeException(e);  // Cannot happen.
        }
    }
}
