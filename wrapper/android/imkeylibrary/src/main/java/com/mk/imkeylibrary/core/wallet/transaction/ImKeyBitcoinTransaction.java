package com.mk.imkeylibrary.core.wallet.transaction;

import org.bitcoinj.core.Address;
import org.bitcoinj.core.Base58;
import org.bitcoinj.core.Coin;
import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.NetworkParameters;
import org.bitcoinj.core.Sha256Hash;
import org.bitcoinj.core.Transaction;
import org.bitcoinj.core.TransactionInput;
import org.bitcoinj.core.TransactionOutPoint;
import org.bitcoinj.core.TransactionOutput;
import org.bitcoinj.core.UnsafeByteArrayOutputStream;
import org.bitcoinj.core.Utils;
import org.bitcoinj.core.VarInt;
import org.bitcoinj.crypto.DeterministicKey;
import org.bitcoinj.crypto.HDKeyDerivation;
import org.bitcoinj.crypto.TransactionSignature;
import org.bitcoinj.params.MainNetParams;
import org.bitcoinj.script.Script;
import org.bitcoinj.script.ScriptBuilder;

import java.io.IOException;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.EccUtil;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.wallet.Btc;
import com.mk.imkeylibrary.core.wallet.Path;
import com.mk.imkeylibrary.core.wallet.Wallet;
import com.mk.imkeylibrary.device.Applet;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class ImKeyBitcoinTransaction extends Wallet {
    private String to;
    private long amount;
    private List<UTXO> outputs;
    private String memo;
    private long fee;
    private int changeIdx;
    private long locktime = 0;
    private String payment;
    private String toDis;
    private String from;
    private String feeDis;
    private HashMap<String, Object> extra;

    // 2730 sat
    private final static long DUST_THRESHOLD = 2730;

    /**
     *
     * @param to
     * @param changeIdx
     * @param amount
     * @param fee
     * @param outputs
     */
    public ImKeyBitcoinTransaction(String to, int changeIdx, long amount, long fee, ArrayList<UTXO> outputs, String payment, String toDis, String from, String feeDis) {
        this.to = to;
        this.amount = amount;
        this.fee = fee;
        this.outputs = outputs;
        this.changeIdx = changeIdx;
        this.payment = payment;
        this.toDis = toDis;
        this.from = from;
        this.feeDis = feeDis;

        if (amount < DUST_THRESHOLD) {
            throw new ImkeyException(Messages.IMKEY_AMOUNT_LESS_THAN_MINIMUM);
        }
    }

    public ImKeyBitcoinTransaction(String to, int changeIdx, long amount, long fee, ArrayList<UTXO> outputs, HashMap<String, Object> extra, String payment, String toDis, String from, String feeDis) {
        this(to, changeIdx, amount, fee, outputs, payment, toDis, from, feeDis);
        this.extra = extra;
    }

    @Override
    public String toString() {
        return "ImKeyBitcoinTransaction{" +
                "to='" + to + '\'' +
                ", amount=" + amount +
                ", outputs=" + outputs +
                ", memo='" + memo + '\'' +
                ", fee=" + fee +
                ", changeIdx=" + changeIdx +
                '}';
    }

    public String getTo() {
        return to;
    }

    public void setTo(String to) {
        this.to = to;
    }

    public long getAmount() {
        return amount;
    }

    public void setAmount(long amount) {
        this.amount = amount;
    }

    public List<UTXO> getOutputs() {
        return outputs;
    }

    public void setOutputs(List<UTXO> outputs) {
        this.outputs = outputs;
    }

    public String getMemo() {
        return memo;
    }

    public void setMemo(String memo) {
        this.memo = memo;
    }

    public long getFee() {
        return fee;
    }

    public void setFee(long fee) {
        this.fee = fee;
    }

    public int getChangeIdx() {
        return changeIdx;
    }

    public void setChangeIdx(int changeIdx) {
        this.changeIdx = changeIdx;
    }

    public long getLocktime() {
        return locktime;
    }

    public static class UTXO {
        private String txHash;
        private int vout;
        private long amount;
        private String address;
        private String scriptPubKey;
        private String derivedPath;
        private long sequence = 4294967295L;


        @Override
        public String toString() {
            return "UTXO{" +
                    "txHash='" + txHash + '\'' +
                    ", vout=" + vout +
                    ", amount=" + amount +
                    ", address='" + address + '\'' +
                    ", scriptPubKey='" + scriptPubKey + '\'' +
                    ", derivedPath='" + derivedPath + '\'' +
                    '}';
        }

        public UTXO(String txHash, int vout, long amount, String address, String scriptPubKey, String derivedPath) {
            this.txHash = txHash;
            this.vout = vout;
            this.amount = amount;
            this.address = address;
            this.scriptPubKey = scriptPubKey;
            this.derivedPath = derivedPath;
        }

        public UTXO(String txHash, int vout, long amount, String address, String scriptPubKey, String derivedPath, long sequence) {
            this.txHash = txHash;
            this.vout = vout;
            this.amount = amount;
            this.address = address;
            this.scriptPubKey = scriptPubKey;
            this.derivedPath = derivedPath;
            this.sequence = sequence;
        }

        public int getVout() {
            return vout;
        }

        public void setVout(int vout) {
            this.vout = vout;
        }

        public long getAmount() {
            return amount;
        }

        public void setAmount(long amount) {
            this.amount = amount;
        }

        public String getAddress() {
            return address;
        }

        public void setAddress(String address) {
            this.address = address;
        }

        public String getTxHash() {
            return txHash;
        }

        public void setTxHash(String txHash) {
            this.txHash = txHash;
        }

        public String getScriptPubKey() {
            return scriptPubKey;
        }

        public void setScriptPubKey(String scriptPubKey) {
            this.scriptPubKey = scriptPubKey;
        }

        public String getDerivedPath() {
            return derivedPath;
        }

        public void setDerivedPath(String derivedPath) {
            this.derivedPath = derivedPath;
        }

        public long getSequence() {
            return sequence;
        }

        public void setSequence(long sequence) {
            this.sequence = sequence;
        }
    }

    /**
     *
     * @param network   value: MAINNET/TESTNET
     * @param pathPrefix
     * @return
     */
    public TransactionSignedResult signTransaction(String network, String pathPrefix) {

        // path check
        Path.checkPath(pathPrefix);
        if(!pathPrefix.endsWith("/")) {
            pathPrefix = pathPrefix + "/";
        }

        // utxo number check
        if(getOutputs().size() > Constants.MAX_UTXO_NUMBER) {
            throw new ImkeyException(Messages.IMKEY_EXCEEDED_MAX_UTXO_NUMBER);
        }
        // select applet
        selectApplet();

        NetworkParameters networkParameters = null;
        if(Constants.MAINNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_MAINNET);
        } else if (Constants.TESTNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_TESTNET);
        } else {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }

        ImkeyTransaction tran = new ImkeyTransaction(networkParameters);

        // get main pubkey
        String mainPubKeyRes = new Btc().getXpubHex(pathPrefix.substring(0,pathPrefix.length()-1), false);
        DeterministicKey mainPubKey = HDKeyDerivation.createMasterPubKeyFromBytes(
                NumericUtil.hexToBytes(mainPubKeyRes.substring(0, 130)),
                NumericUtil.hexToBytes(mainPubKeyRes.substring(130)));

        List<byte[]> pubKeys = new ArrayList<>();
        // get pubkey
        for (int i = 0; i < getOutputs().size(); i++) {

            DeterministicKey childPubKey = EccUtil.deriveChildKeyFromPublic(mainPubKey, outputs.get(i).derivedPath);
            byte[] pubKey = childPubKey.getPubKey();

            // verify address
            if(!verifyAddr(getOutputs().get(i).getAddress(), networkParameters, pubKey)) {
                throw new ImkeyException(Messages.IMKEY_ADDRESS_MISMATCH_WITH_PATH);
            }

            pubKeys.add(pubKey);
        }

        long totalAmount = 0L;

        for (UTXO output : getOutputs()) {
            totalAmount += output.getAmount();
        }

        if (totalAmount < getAmount()) {
            throw new ImkeyException(Messages.IMKEY_INSUFFICIENT_FUNDS);
        }

        //add send to output
        tran.addOutput(Coin.valueOf(getAmount()), Address.fromBase58(networkParameters, getTo()));

        //add change output
        long changeAmount = totalAmount - (getAmount() + getFee());


        if (changeAmount >= DUST_THRESHOLD) {
            // get change address
            String changePath = pathPrefix + "1/" + String.valueOf(changeIdx);
            String address = new Btc().getAddress(networkParameters.getAddressHeader(), changePath);
            tran.addOutput(Coin.valueOf(changeAmount), Address.fromBase58(networkParameters, address));
        }

        // add the OP_RETURN
        if (this.extra != null) {
            String opReturn = (String) this.extra.get("opReturn");
            byte[] opReturnBytes = NumericUtil.hexToBytes(opReturn);
            tran.addOutput(Coin.ZERO, ScriptBuilder.createOpReturnScript(opReturnBytes));
        }

        //output data serialize
        byte[] serivalizeData = tran.serializeTransaction(Transaction.SigHash.ALL, false);
        //set utxo length
        serivalizeData[4] = (byte)outputs.size();
        String signedHex = NumericUtil.bytesToHex(serivalizeData);

        byte[] bytes = ByteUtil.longToByteArray(fee);
        signedHex += ByteUtil.byteArrayToHexString(bytes);

        //get address version
        String version = Integer.toHexString(Address.fromBase58(networkParameters, getTo()).getVersion());
        if(version.length()%2 !=0) {
            version = "0" + version;
        }
        signedHex += version;

        // prepare
        byte[] signBytes = ByteUtil.hexStringToByteArray(signedHex);
        byte[] SignBytesWtl = ByteUtil.concat(new byte[]{(byte)0x01}, ByteUtil.concat(new byte[]{(byte)signBytes.length},signBytes));
        byte[] hashData  = Sha256Hash.hashTwice(SignBytesWtl);
        byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
        byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signature.length},signature));
        byte[] apduPack = ByteUtil.concat(signatureWtl, SignBytesWtl);

        String res = Ble.getInstance().sendApdu(Apdu.btcPrepareInput((byte)0x00, NumericUtil.bytesToHex(apduPack)), Constants.SEND_SIGN_PRE_APDU_TIMEOUT);
        Apdu.checkResponse(res);

        for (UTXO output : getOutputs()) {
            tran.addInput(Sha256Hash.wrap(output.getTxHash()), output.getVout(), new Script(NumericUtil.hexToBytes(output.getScriptPubKey())));
        }
        ImkeyTransaction tranTemp = new ImkeyTransaction(networkParameters);

        int count = (getOutputs().size() -1)/Constants.EACH_ROUND_NUMBER + 1;
        byte[] tempPrepareInput = null;
        byte[] tempData = null;
        for(int i = 0; i < count; i++){
            for (int x = 0; x < getOutputs().size(); x++){
                if((x >= i * Constants.EACH_ROUND_NUMBER) && (x < (i + 1) * Constants.EACH_ROUND_NUMBER)){
                    tranTemp.addInput(Sha256Hash.wrap(getOutputs().get(x).getTxHash()), getOutputs().get(x).getVout(), new Script(NumericUtil.hexToBytes(getOutputs().get(x).getScriptPubKey())));
                }else {
                    tranTemp.addInput(Sha256Hash.wrap(getOutputs().get(x).getTxHash()), getOutputs().get(x).getVout(), new Script(new byte[0]));
                }

                tempPrepareInput = tranTemp.serializeTransaction(Transaction.SigHash.ALL, false);
                tempData = new byte[tempPrepareInput.length - 13];
                System.arraycopy(tempPrepareInput, 4, tempData, 0, tempData.length);
                tempData[0] = (byte)x;
                res = Ble.getInstance().sendApdu(Apdu.btcPrepareInput((byte)0x80, NumericUtil.bytesToHex(tempData)));
                Apdu.checkResponse(res);
                tranTemp.clearInputs();
            }

            for (int y = i * Constants.EACH_ROUND_NUMBER; y < (i + 1) * Constants.EACH_ROUND_NUMBER; y++){
                if (y >= pubKeys.size()){
                    break;
                }
                UTXO output = getOutputs().get(y);
                byte[] pubkeyBytes;
                // Uncompressed format
                if (output.getAddress().equals(getAddressFromPubKey(networkParameters, pubKeys.get(y)))){
                    pubkeyBytes = pubKeys.get(y);
                    // Compressed format
                } else {
                    pubkeyBytes = NumericUtil.hexToBytes(calComprsPub(NumericUtil.bytesToHex(pubKeys.get(y))));
                }

                TransactionInput transactionInput = tran.getInput(y);

                String path = pathPrefix + outputs.get(y).derivedPath;
                String sig = Apdu.btcSign(y, Apdu.Hash_ALL, path);
                String sigRes = Ble.getInstance().sendApdu(sig);
                LogUtil.d("signResult" + y + "：" + sigRes);//响应报文为签名结果，格式为L|R|S|V|，66字节，其中L为1个字节，R、S分别为32字节，V为1个字节（27或28）
                // check sign result
                Apdu.checkResponse(sigRes);
                String r = sigRes.substring(2, 66);
                String s = sigRes.substring(66, 130);

                LogUtil.d("\n********************");
                LogUtil.d(y + " r:" + r);
                LogUtil.d(y + " s:" + s);

                TransactionSignature txSig = new TransactionSignature(new BigInteger(r,16), getLowS(new BigInteger(s,16)));
                transactionInput.setScriptSig(com.mk.imkeylibrary.core.wallet.script.ScriptBuilder.createInputScript(txSig, pubkeyBytes));
            }

        }

        signedHex = NumericUtil.bytesToHex(tran.bitcoinSerialize());
        String txHash = NumericUtil.beBigEndianHex(Hash.sha256(Hash.sha256(signedHex)));
        return new TransactionSignedResult(signedHex, txHash);
    }

    /**
     *
     * @param network
     * @param pathPrefix
     * @return
     */
    public TransactionSignedResult signSegWitTransaction(String network, String pathPrefix) {

        // path check
        Path.checkPath(pathPrefix);
        if(!pathPrefix.endsWith("/")) {
            pathPrefix = pathPrefix + "/";
        }
        // select applet
        selectApplet();

        NetworkParameters networkParameters = null;
        if(Constants.MAINNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_MAINNET);
        } else if (Constants.TESTNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_TESTNET);
        } else {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }

        ImkeyTransaction tran = new ImkeyTransaction(MainNetParams.get());

        // get main pubkey
        String mainPubKeyRes = new Btc().getXpubHex(pathPrefix.substring(0,pathPrefix.length()-1), false);
        DeterministicKey mainPubKey = HDKeyDerivation.createMasterPubKeyFromBytes(
                NumericUtil.hexToBytes(mainPubKeyRes.substring(0, 130)),
                NumericUtil.hexToBytes(mainPubKeyRes.substring(130)));

        List<byte[]> pubKeys = new ArrayList<>();

        // get pubkey
        for (int i = 0; i < getOutputs().size(); i++) {

            DeterministicKey childPubKey = EccUtil.deriveChildKeyFromPublic(mainPubKey, outputs.get(i).derivedPath);
            byte[] pubKey = childPubKey.getPubKey();

            // verify address
            if(!verifyAddrSegwit(getOutputs().get(i).getAddress(), networkParameters, pubKey)) {
                throw new ImkeyException(Messages.IMKEY_ADDRESS_MISMATCH_WITH_PATH);
            }

            pubKeys.add(pubKey);
        }

        long[] inputvalue = new long[getOutputs().size()];

        long totalAmount = 0L;
        boolean hasChange = false;

        for (UTXO output : getOutputs()) {
            totalAmount += output.getAmount();
        }

        if (totalAmount < getAmount()) {
            throw new ImkeyException(Messages.IMKEY_INSUFFICIENT_FUNDS);
        }
        long changeAmount = totalAmount - (getAmount() + getFee());

        Address toAddress = Address.fromBase58(networkParameters, to);
        Script targetScriptPubKey;
        String version;
        if (toAddress.isP2SHAddress()) {
            targetScriptPubKey = ScriptBuilder.createP2SHOutputScript(toAddress.getHash160());
            //get version prefix
            version = Integer.toHexString(networkParameters.getP2SHHeader());
        } else {
            targetScriptPubKey = ScriptBuilder.createOutputScript(toAddress);
            version = Integer.toHexString(networkParameters.getAddressHeader());
        }

        // get change address
        String changePath = pathPrefix + "1/" + String.valueOf(changeIdx);
        String changeAddress = new Btc().getSegWitAddress(networkParameters.getP2SHHeader(), changePath);
        LogUtil.d("........................change address:" + changeAddress);
        Script changeScriptPubKey = ScriptBuilder.createP2SHOutputScript(getHash160(changeAddress));

        // construct output
        int outputSize = 0;
        tran.addOutput(Coin.valueOf(getAmount()), targetScriptPubKey);
        outputSize++;

        if (changeAmount >= DUST_THRESHOLD) {
            hasChange = true;
            tran.addOutput(Coin.valueOf(changeAmount), changeScriptPubKey);
            outputSize++;
        }

        byte[] opReturnProgram = null;
        if (extra != null) {
            String opReturn = (String) this.extra.get("opReturn");
            byte[] opReturnBytes = NumericUtil.hexToBytes(opReturn);
            opReturnProgram = ScriptBuilder.createOpReturnScript(opReturnBytes).getProgram();
            TransactionOutput opReturnOutput = new TransactionOutput(networkParameters, null, Coin.ZERO, opReturnProgram);
            tran.addOutput(opReturnOutput);
            outputSize++;
        }

        //serialize transaction data
        byte[] seralizeData = tran.serializeSegWitTransaction(Transaction.SigHash.ALL, false, tran.getInputs().size(), outputSize, inputvalue);
        seralizeData[4] = (byte)outputs.size();
        String signedHexraw = NumericUtil.bytesToHex(seralizeData);

        byte[] bytes = ByteUtil.longToByteArray(fee);
        signedHexraw += ByteUtil.byteArrayToHexString(bytes);

        if(version.length()%2 !=0) {
            version = "0" + version;
        }
        signedHexraw += version;
        byte[] signBytes = ByteUtil.hexStringToByteArray(signedHexraw);
        byte[] SignBytesWtl = ByteUtil.concat(new byte[]{(byte)0x01}, ByteUtil.concat(new byte[]{(byte)signBytes.length},signBytes));
        byte[] hashData  = Sha256Hash.hashTwice(SignBytesWtl);
        byte[] signatureRaw = Wallet.signPackage(Sha256Hash.wrap(hashData));
        byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signatureRaw.length},signatureRaw));
        byte[] apduPack = ByteUtil.concat(signatureWtl, SignBytesWtl);

        // prepare data
        List<String> pres = Apdu.btcSegwitPrepare((byte)0x00, apduPack);
        LogUtil.d("btc prepare....");
        for (int i = 0; i < pres.size(); i++) {
            String apdu = pres.get(i);
            int timeout = Constants.SENT_APDU_TIMEOUT;
            if (i == (pres.size()-1)) {
                timeout = Constants.SEND_SIGN_PRE_APDU_TIMEOUT;
            }
            String res = Ble.getInstance().sendApdu(apdu, timeout);
            Apdu.checkResponse(res);
        }

        UnsafeByteArrayOutputStream txHashVoutStream = new UnsafeByteArrayOutputStream();
        UnsafeByteArrayOutputStream sequenceStream = new UnsafeByteArrayOutputStream();

        // construct input
        for (int i = 0; i < getOutputs().size(); i++) {
            // get pubkey
            ECKey key = EccUtil.getECKeyFromPublicOnly(pubKeys.get(i));
            UTXO output = getOutputs().get(i);
            // create scriptCode
            byte[] scriptCode = NumericUtil.hexToBytes(String.format("0x76a914%s88ac", NumericUtil.bytesToHex(key.getPubKeyHash())));
            // create input
            TransactionOutPoint outPoint = new TransactionOutPoint(networkParameters, output.vout, Sha256Hash.wrap(output.txHash));
            TransactionInput input = new TransactionInput(networkParameters, null, scriptCode, outPoint, Coin.valueOf(output.getAmount()));
            inputvalue[i] = output.getAmount();
            // add input
            tran.addInput(input);
            try {
                txHashVoutStream.write(NumericUtil.reverseBytes(NumericUtil.hexToBytes(output.txHash)));
                Utils.uint32ToByteStreamLE(output.getVout(), txHashVoutStream);
                Utils.uint32ToByteStreamLE(output.getSequence(), sequenceStream);
            } catch (IOException e) {
                throw new ImkeyException("OutputStream error");
            }
        }

        List<String> txHashVoutApduList =  Apdu.btcSegwitPrepare((byte)0x40, txHashVoutStream.toByteArray());
        List<String> sequenceApduList =  Apdu.btcSegwitPrepare((byte)0x80, sequenceStream.toByteArray());
        txHashVoutApduList.addAll(sequenceApduList);
        for(String apdu : txHashVoutApduList){
            Apdu.checkResponse(Ble.getInstance().sendApdu(apdu, Constants.SENT_APDU_TIMEOUT));
        }

        try {
            // calc witnesses and redeemScripts
            List<byte[]> witnesses = new ArrayList<>();
            List<String> redeemScripts = new ArrayList<>();
            for (int i = 0; i < getOutputs().size(); i++) {
                UnsafeByteArrayOutputStream stream = new UnsafeByteArrayOutputStream();
                UTXO utxo = getOutputs().get(i);
                ECKey key = EccUtil.getECKeyFromPublicOnly(pubKeys.get(i));
                String redeemScript = String.format("0014%s", NumericUtil.bytesToHex(key.getPubKeyHash()));
                redeemScripts.add(redeemScript);

                //txHash
                stream.write(NumericUtil.reverseBytes(NumericUtil.hexToBytes(utxo.getTxHash())));
                //vout
                Utils.uint32ToByteStreamLE(utxo.getVout(), stream);
                //lock script
                byte[] scriptCode = NumericUtil.hexToBytes(String.format("0x76a914%s88ac", NumericUtil.bytesToHex(key.getPubKeyHash())));
                byte[] scriptCodeTemp = new byte[scriptCode.length + 1];
                scriptCodeTemp[0] = (byte) scriptCode.length;
                System.arraycopy(scriptCode, 0, scriptCodeTemp, 1, scriptCode.length);
                stream.write(scriptCodeTemp);
                //UTXO amount
                Utils.uint64ToByteStreamLE(BigInteger.valueOf(utxo.getAmount()), stream);
                //sequence
                Utils.uint32ToByteStreamLE(utxo.getSequence(), stream);
                byte[] signSerializeBytes = stream.toByteArray();
                byte[] tempBytes = new byte[signSerializeBytes.length + 1];
                tempBytes[0] = (byte) signSerializeBytes.length;
                System.arraycopy(signSerializeBytes, 0, tempBytes, 1, signSerializeBytes.length);

                //address
                String path = pathPrefix + outputs.get(i).derivedPath;
                byte[] pathBytesTemp = new byte[path.getBytes().length + 1];
                pathBytesTemp[0] = (byte)path.getBytes().length;
                System.arraycopy(path.getBytes(), 0, pathBytesTemp, 1, path.getBytes().length);
                byte[] data = ByteUtil.concat(tempBytes, pathBytesTemp);
                //sign
                String sigApdu = Apdu.btcSegwitSign(i==getOutputs().size()-1, Apdu.Hash_ALL, data);
                String sigRes = Ble.getInstance().sendApdu(sigApdu);
                Apdu.checkResponse(sigRes);
                LogUtil.d("signResult" + i + "：" + sigRes);//响应报文为签名结果，格式为L|R|S|V|，66字节，其中L为1个字节，R、S分别为32字节，V为1个字节（27或28）
                String r = sigRes.substring(2, 66);
                String s = sigRes.substring(66, 130);
                TransactionSignature signature = new TransactionSignature(new BigInteger(r,16), getLowS(new BigInteger(s,16)));
                byte hashType = 0x01;
                byte[] sig = ByteUtil.concat(signature.encodeToDER(), new byte[]{hashType});
                witnesses.add(sig);
            }

            UnsafeByteArrayOutputStream[] serialStreams = new UnsafeByteArrayOutputStream[]{
                    new UnsafeByteArrayOutputStream(), new UnsafeByteArrayOutputStream()
            };
            UnsafeByteArrayOutputStream stream = new UnsafeByteArrayOutputStream();
            for (int idx = 0; idx < 2; idx++) {
                stream = serialStreams[idx];
                Utils.uint32ToByteStreamLE(2L, stream); // version
                if (idx == 0) {
                    stream.write(0x00); // maker
                    stream.write(0x01); // flag
                }
                // inputs
                stream.write(new VarInt(getOutputs().size()).encode());
                for (int i = 0; i < getOutputs().size(); i++) {
                    UTXO utxo = getOutputs().get(i);
                    stream.write(NumericUtil.reverseBytes(NumericUtil.hexToBytes(utxo.txHash)));
                    Utils.uint32ToByteStreamLE(utxo.getVout(), stream);

                    // the length of byte array that follows, and this length is used by OP_PUSHDATA1
                    stream.write(0x17);
                    // the length of byte array that follows, and this length is used by cutting array
                    stream.write(0x16);
                    stream.write(NumericUtil.hexToBytes(redeemScripts.get(i)));
                    Utils.uint32ToByteStreamLE(utxo.getSequence(), stream);
                }

                // outputs
                // outputs size, In our case, it just contains to address and change address
                stream.write(new VarInt(outputSize).encode());
                //stream.write(new VarInt(2).encode());
                Utils.uint64ToByteStreamLE(BigInteger.valueOf(amount), stream);
                stream.write(new VarInt(targetScriptPubKey.getProgram().length).encode());
                stream.write(targetScriptPubKey.getProgram());
                if (hasChange) {
                    Utils.uint64ToByteStreamLE(BigInteger.valueOf(changeAmount), stream);
                    stream.write(new VarInt(changeScriptPubKey.getProgram().length).encode());
                    stream.write(changeScriptPubKey.getProgram());
                }

                if (opReturnProgram != null) {
                    Utils.uint64ToByteStreamLE(BigInteger.valueOf(0), stream);
                    stream.write(new VarInt(opReturnProgram.length).encode());
                    stream.write(opReturnProgram);
                }

                if (idx == 0) {
                    for (int i = 0; i < witnesses.size(); i++) {
                        //ECKey ecKey = ECKey.fromPrivate(prvKeys.get(i));
                        ECKey ecKey = EccUtil.getECKeyFromPublicOnly(pubKeys.get(i));
                        byte[] wit = witnesses.get(i);
                        stream.write(new VarInt(2).encode());
                        stream.write(new VarInt(wit.length).encode());
                        stream.write(wit);
                        stream.write(new VarInt(ecKey.getPubKey().length).encode());
                        stream.write(ecKey.getPubKey());
                    }
                }

                Utils.uint32ToByteStreamLE(locktime, stream);
            }
            byte[] signed = serialStreams[0].toByteArray();
            String signedHex = NumericUtil.bytesToHex(signed);
            String wtxID = NumericUtil.bytesToHex(Sha256Hash.hashTwice(signed));
            wtxID = NumericUtil.beBigEndianHex(wtxID);
            String txHash = NumericUtil.bytesToHex(Sha256Hash.hashTwice(serialStreams[1].toByteArray()));
            txHash = NumericUtil.beBigEndianHex(txHash);
            return new TransactionSignedResult(signedHex, txHash, wtxID);
        } catch (IOException ex) {
            throw new ImkeyException("OutputStream error");
        }
    }


    @Override
    protected String getAid() {
        return Applet.BTC_AID;
    }

    protected BigInteger getLowS(BigInteger s){
        if(s.compareTo(Constants.HALF_CURVE_ORDER) > 0) {
            s = Constants.CURVE_N.subtract(s);
        }
        return s;
    }

    private byte[] getHash160(String address) {
        byte[] versionAndDataBytes = Base58.decodeChecked(address);
        byte[] bytes = new byte[versionAndDataBytes.length - 1];
        System.arraycopy(versionAndDataBytes, 1, bytes, 0, versionAndDataBytes.length - 1);
        return bytes;
    }

    protected String getAddressFromPubKey(NetworkParameters params, byte[] pubKey) {
        byte[] getPubKeyHash = Utils.sha256hash160(pubKey);
        return new Address(params, getPubKeyHash).toBase58();
    }

    protected boolean verifyAddr(String addr, NetworkParameters params, byte[] pubKey) {
        // 非压缩格式
        if(addr.equals(getAddressFromPubKey(params, pubKey))) {
            return true;
        }
        // 压缩格式
        if(addr.equals(getAddressFromPubKey(params, NumericUtil.hexToBytes(calComprsPub(NumericUtil.bytesToHex(pubKey)))))) {
            return true;
        }
        return false;
    }

    protected boolean verifyAddrSegwit(String addr, NetworkParameters params, byte[] pubKey) {

        String comprsPub = ByteUtil.byteArrayToHexString(pubKey);
        if(pubKey.length == 65) {
            comprsPub = calComprsPub(ByteUtil.byteArrayToHexString(pubKey));
        }

        String addrSegwit = new Wallet().calcSegWitAddress(params.getP2SHHeader(), comprsPub);

        if(addr.equals(addrSegwit)) {
            return true;
        }
        return false;
    }
}
