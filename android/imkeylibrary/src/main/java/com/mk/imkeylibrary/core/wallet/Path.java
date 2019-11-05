package com.mk.imkeylibrary.core.wallet;

import java.util.regex.Pattern;

import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;

public class Path {
    public static final String BTC_PATH_PREFIX = "m/44'/0'/0'/";
    public final static String BITCOIN_TESTNET_PATH = "m/44'/1'/0'";
    public static final String BTC_SEGWIT_PATH_PREFIX = "m/49'/0'/0'/";
    public final static String BITCOIN_SEGWIT_TESTNET_PATH = "m/49'/1'/0'/";
    public static final String ETH_LEDGER = "m/44'/60'/0'/0/0";
    public static final String EOS_LEDGER = "m/44'/194'/0'/0/0";
    public static final String COSMOS_LEDGER = "m/44'/118'/0'/0/0";

    public static void checkPath(String path) {

        // 深度大于1，小于10，目前规范是5
        if(path.split("/").length < 2 || path.split("/").length > 10) {
            throw new ImkeyException(Messages.IMKEY_PATH_ILLEGAL);
        }

        // 长度不超过100个字符
        if(path.length() > 100) {
            throw new ImkeyException(Messages.IMKEY_PATH_ILLEGAL);
        }

        // 以m/开头
        String regEx = "^m/[0-9'/]+$";
        if (!Pattern.matches(regEx, path)) {
            throw new ImkeyException(Messages.IMKEY_PATH_ILLEGAL);
        }

    }


}
