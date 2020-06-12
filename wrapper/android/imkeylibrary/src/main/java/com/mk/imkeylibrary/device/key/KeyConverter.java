package com.mk.imkeylibrary.device.key;


import org.spongycastle.jcajce.provider.asymmetric.ec.BCECPrivateKey;
import org.spongycastle.jce.provider.BouncyCastleProvider;

import java.math.BigInteger;
import java.security.KeyFactory;
import java.security.NoSuchAlgorithmException;
import java.security.NoSuchProviderException;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.Security;
import java.security.spec.ECPoint;
import java.security.spec.InvalidKeySpecException;

public class KeyConverter {
    static {
        Security.addProvider(new org.spongycastle.jce.provider.BouncyCastleProvider());
    }

    private static ECPoint convertECPoint(org.spongycastle.math.ec.ECPoint g) {
        return new ECPoint(g.getXCoord().toBigInteger(), g.getYCoord().toBigInteger());
    }

    public static PrivateKey getPrivKey(byte[] privKey) throws NoSuchProviderException, NoSuchAlgorithmException, InvalidKeySpecException {

        org.spongycastle.jce.spec.ECNamedCurveParameterSpec secp256k1 = org.spongycastle.jce.ECNamedCurveTable.getParameterSpec("secp256k1");
        org.spongycastle.jce.spec.ECPrivateKeySpec privSpec = new org.spongycastle.jce.spec.ECPrivateKeySpec(new BigInteger(1, privKey), secp256k1);

        KeyFactory keyFactory = KeyFactory.getInstance("EC", BouncyCastleProvider.PROVIDER_NAME);
        BCECPrivateKey bcpriv = (BCECPrivateKey) keyFactory.generatePrivate(privSpec);
        return bcpriv;

    }

    public static PublicKey getPubKey(byte[] pubKey) throws NoSuchProviderException, NoSuchAlgorithmException, InvalidKeySpecException {

        byte[] xByte = new byte[32];
        byte[] yByte = new byte[32];
        System.arraycopy(pubKey, 1, xByte,0,32);
        System.arraycopy(pubKey, 33, yByte,0,32);

        BigInteger x = new BigInteger(1, xByte);
        BigInteger y = new BigInteger(1, yByte);

        org.spongycastle.jce.spec.ECNamedCurveParameterSpec secp256k1 = org.spongycastle.jce.ECNamedCurveTable.getParameterSpec("secp256k1");
        KeyFactory keyFactory = KeyFactory.getInstance("EC", BouncyCastleProvider.PROVIDER_NAME);
        org.spongycastle.math.ec.ECPoint ecpubPoint = new org.spongycastle.math.ec.custom.sec.SecP256K1Curve().createPoint(x, y);
        org.spongycastle.jcajce.provider.asymmetric.ec.BCECPublicKey publicKey = (org.spongycastle.jcajce.provider.asymmetric.ec.BCECPublicKey) keyFactory.generatePublic(new org.spongycastle.jce.spec.ECPublicKeySpec(ecpubPoint, secp256k1));

        return publicKey;
    }

}
