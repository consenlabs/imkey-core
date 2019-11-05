package com.mk.imkeylibrary.core.foundation.crypto;

import org.bitcoinj.core.ECKey;
import org.bitcoinj.crypto.ChildNumber;
import org.bitcoinj.crypto.DeterministicKey;
import org.bitcoinj.crypto.HDKeyDerivation;

import java.math.BigInteger;

import com.mk.imkeylibrary.common.Constants;

public class EccUtil {

    /**
     * get compressed public key from uncompressed on since segwit requires compressed public key only
     * Uncompressed public key is:
     * 0x04 + x-coordinate + y-coordinate
     * Compressed public key is:
     * 0x02 + x-coordinate if y is even
     * 0x03 + x-coordinate if y is odd
     */
    public static  ECKey getCompressECKey(byte[] pubKeyUncompress) {

        byte[] desPubKey = new byte[33];
        System.arraycopy(pubKeyUncompress, 33, desPubKey, 1, 32);
        if ((desPubKey[32] % 2) != 0) desPubKey[0] = 0x03;
        else desPubKey[0] = 0x02;

        System.arraycopy(pubKeyUncompress, 1, desPubKey, 1, 32);
        return ECKey.fromPublicOnly(desPubKey);
    }

    public static  ECKey getECKeyFromPublicOnly(byte[] pubKey) {
        if(pubKey.length == 33) {
            return ECKey.fromPublicOnly(pubKey);
        } else {
            return getCompressECKey(pubKey);
        }
    }

    public static BigInteger getLowS(BigInteger s){
        if(s.compareTo(Constants.HALF_CURVE_ORDER) > 0) {
            s = Constants.CURVE_N.subtract(s);
        }
        return s;
    }

    public static DeterministicKey deriveChildKeyFromPublic(DeterministicKey publicKey, String derivedPath) {

        String[] pathIdxs = derivedPath.replace('/', ' ').split(" ");
        for(int i=0; i<pathIdxs.length; i++) {
            publicKey = HDKeyDerivation.deriveChildKey(publicKey, new ChildNumber(Integer.parseInt(pathIdxs[i]), false));
        }
        return publicKey;
    }
}
