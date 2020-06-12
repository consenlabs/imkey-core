package com.mk.imkeylibrary.core.foundation.crypto;


import org.spongycastle.jce.provider.BouncyCastleProvider;

import java.security.Security;

public class SecurityProvider {

    private static String PROVIDER_NAME = BouncyCastleProvider.PROVIDER_NAME;
    private static boolean ISDECIDE = false;
    public static String getProvierName(){
        if (ISDECIDE && Security.getProvider(PROVIDER_NAME)!=null){
            return PROVIDER_NAME;
        }else{
            Security.addProvider(new BouncyCastleProvider());
            ISDECIDE=true;
            return PROVIDER_NAME;
        }

    }
}
