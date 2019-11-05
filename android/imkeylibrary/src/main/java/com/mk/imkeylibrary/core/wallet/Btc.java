package com.mk.imkeylibrary.core.wallet;

import org.bitcoinj.core.Base58;
import org.bitcoinj.crypto.ChildNumber;

import java.nio.ByteBuffer;
import java.util.List;

import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.NumericUtil;

public class Btc extends Wallet {

    /**
     * @param version mainnet：76067358(0x0488B21E) testnet：70617039(0x043587CF)
     * @param path
     * @return
     */
    public String getXpub(int version, String path) {
        // path校验
        Path.checkPath(path);

        selectApplet();
        String xpubHex = getXpubHex(path, true);
        String parentXpubHex = getXpubHex(getParentPath(path), true);
        String parentComprsPub = calComprsPub(parentXpubHex.substring(0, 130));

        ByteBuffer ser = ByteBuffer.allocate(78);
        ser.putInt(version);
        ser.put((byte) getDepth(path));
        ser.putInt(getFingerprint(NumericUtil.hexToBytes(parentComprsPub)));
        List<ChildNumber> childNumberList = generatePath(path);
        ser.putInt(childNumberList.get(childNumberList.size() - 1).i());
        ser.put(NumericUtil.hexToBytes(xpubHex.substring(130, 194)));
        ser.put(NumericUtil.hexToBytes(calComprsPub(xpubHex.substring(0, 130))));
        return Base58.encode(addChecksum(ser.array()));
    }

    public String getAddress(int version, String path) {
        // path校验
        Path.checkPath(path);

        selectApplet();
        String xpub = getXpubHex(path, true);
        String comprsPub = calComprsPub(xpub.substring(0, 130));
        return pub2Address(version, comprsPub);
    }

    public String displayAddress(int version, String path) {
        // path校验
        Path.checkPath(path);

        String mainAddr = getAddress(version, path);

        String apduCoinReg = Apdu.btcCoinReg(mainAddr.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);
        return mainAddr;
    }

    public String getSegWitAddress(int version, String path) {
        // path校验
        Path.checkPath(path);

        selectApplet();
        if (version < 0 || version >= 256) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }
        String xpub = getXpubHex(path, true);
        String comprsPub = calComprsPub(xpub.substring(0, 130));
        return calcSegWitAddress(version, comprsPub);
    }

    public String displaySegWitAddress(int version, String path) {
        // path校验
        Path.checkPath(path);

        String mainAddr = getSegWitAddress(version, path);

        String apduCoinReg = Apdu.btcCoinReg(mainAddr.getBytes());
        String res = sendApdu(apduCoinReg);
        Apdu.checkResponse(res);
        return mainAddr;
    }

    @Override
    protected String getAid() {
        return Applet.BTC_AID;
    }
}
