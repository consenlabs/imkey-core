package com.mk.imkeylibrary.core.wallet;

import org.bitcoinj.core.Base58;
import org.spongycastle.crypto.digests.RIPEMD160Digest;

import java.nio.charset.Charset;
import java.util.Arrays;

import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.wallet.transaction.EOSSign;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class Eos extends Wallet {

    public String getPubKey(String path) {
        // path校验
        Path.checkPath(path);

        selectApplet();
        String xpub = getEosXpubHex(path, true);
        String comprsPub = calComprsPub(xpub);
        byte[] pubKeyData = NumericUtil.hexToBytes(comprsPub);
        RIPEMD160Digest digest = new RIPEMD160Digest();
        digest.update(pubKeyData, 0, pubKeyData.length);
        byte[] out = new byte[20];
        digest.doFinal(out, 0);
        byte[] checksumBytes = Arrays.copyOfRange(out, 0, 4);

        pubKeyData = ByteUtil.concat(pubKeyData, checksumBytes);
        String eosPK = "EOS" + Base58.encode(pubKeyData);
        return eosPK;
    }

    public String displayPubKey(String path) {
        // path校验
        Path.checkPath(path);

        String eosPK = getPubKey(path);

        String apduCoinReg = Apdu.eosCoinReg(eosPK.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);

        return eosPK;
    }

    @Override
    protected String getAid() {
        return Applet.EOS_AID;
    }

    public static String eosEcSign(String data, boolean isHex, String pubKey, String path) {
        // path校验
        Path.checkPath(path);

        byte[] dataHashed;
        if (isHex) {
            dataHashed = NumericUtil.hexToBytes(data);
        } else {
            byte[] dataBytes = data.getBytes(Charset.forName("UTF-8"));
            dataHashed = Hash.sha256(dataBytes);
        }
        byte[] dataHashPrefix = {0x01,0x20};  //TL
        byte[] pathBytes = path.getBytes();
        byte[] pathPrefix = {0x02,(byte) pathBytes.length};  //TL;
        byte[] txPack = ByteUtil.concat(ByteUtil.concat(dataHashPrefix, dataHashed), ByteUtil.concat(pathPrefix, pathBytes));
        String signed = EOSSign.signMessage(txPack, pubKey, dataHashed);
        return signed;
    }

}
