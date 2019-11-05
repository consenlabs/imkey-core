package com.mk.imkeylibrary.core.wallet;

import org.bitcoinj.core.AddressFormatException;
import org.bitcoinj.core.ECKey;

import java.io.ByteArrayOutputStream;

import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.EccUtil;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.utils.Bech32;
import com.mk.imkeylibrary.utils.NumericUtil;

public class Cosmos extends Wallet {

    public String getAddress(String path) {
        // path校验
        Path.checkPath(path);

        selectApplet();
        String xpub = getCosmosXpubHex(path, true);

        ECKey ecKey = EccUtil.getCompressECKey(NumericUtil.hexToBytes(xpub));
        byte[] pubKeyHash = ecKey.getPubKeyHash();
        return Bech32.encode("cosmos", convertBits(pubKeyHash, 0, pubKeyHash.length, 8, 5, true));
    }

    public String displayAddress(String path) {
        // path校验
        Path.checkPath(path);

        String mainAddr = getAddress(path);
        String apduCoinReg = Apdu.cosmosCoinReg(mainAddr.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);
        return mainAddr;
    }

    private static byte[] convertBits(final byte[] in, final int inStart, final int inLen, final int fromBits,
                                      final int toBits, final boolean pad) throws AddressFormatException {
        int acc = 0;
        int bits = 0;
        ByteArrayOutputStream out = new ByteArrayOutputStream(64);
        final int maxv = (1 << toBits) - 1;
        final int max_acc = (1 << (fromBits + toBits - 1)) - 1;
        for (int i = 0; i < inLen; i++) {
            int value = in[i + inStart] & 0xff;
            if ((value >>> fromBits) != 0) {
                throw new AddressFormatException(
                        String.format("Input value '%X' exceeds '%d' bit size", value, fromBits));
            }
            acc = ((acc << fromBits) | value) & max_acc;
            bits += fromBits;
            while (bits >= toBits) {
                bits -= toBits;
                out.write((acc >>> bits) & maxv);
            }
        }
        if (pad) {
            if (bits > 0)
                out.write((acc << (toBits - bits)) & maxv);
        } else if (bits >= fromBits || ((acc << (toBits - bits)) & maxv) != 0) {
            throw new AddressFormatException("Could not convert bits, invalid padding");
        }
        return out.toByteArray();
    }

    @Override
    protected String getAid() {
        return Applet.COSMOS_AID;
    }
}
