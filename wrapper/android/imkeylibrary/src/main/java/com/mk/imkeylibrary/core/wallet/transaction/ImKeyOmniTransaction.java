package com.mk.imkeylibrary.core.wallet.transaction;

import org.bitcoinj.core.Address;
import org.bitcoinj.core.Coin;
import org.bitcoinj.core.ECKey;
import org.bitcoinj.core.NetworkParameters;
import org.bitcoinj.core.Sha256Hash;
import org.bitcoinj.core.Transaction;
import org.bitcoinj.core.TransactionInput;
import org.bitcoinj.core.TransactionOutPoint;
import org.bitcoinj.core.UnsafeByteArrayOutputStream;
import org.bitcoinj.core.Utils;
import org.bitcoinj.core.VarInt;
import org.bitcoinj.crypto.DeterministicKey;
import org.bitcoinj.crypto.HDKeyDerivation;
import org.bitcoinj.crypto.TransactionSignature;
import org.bitcoinj.params.MainNetParams;
import org.bitcoinj.script.Script;
import org.bitcoinj.script.ScriptBuilder;
import org.bitcoinj.script.ScriptOpCodes;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.math.BigInteger;
import java.util.ArrayList;
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
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

public class ImKeyOmniTransaction extends ImKeyBitcoinTransaction {

  private static final Coin MIN_NONDUST_OUTPUT = Coin.valueOf(546L);

  private int propertyId;
  private long totalBtcAmount;
  private long btcChangeAmount;
  private Address sender, receiver;

  private Script omniExtraScript;
  private final List<byte[]> witnesses = new ArrayList<>();
  private final List<String> redeemScripts = new ArrayList<>();
  private byte[] receiverScriptPubKey;
  private byte[] senderScriptPubKey;
  private List<Output> outputs;
  private byte[] hashOutputs;


  public ImKeyOmniTransaction(String to, long amount, long fee, int propertyId, ArrayList<UTXO> outputs, String payment, String toDis, String from, String feeDis) {
    super(to, 0, amount, fee, outputs, payment, toDis, from, feeDis);

    this.propertyId = propertyId;
    totalBtcAmount = calcTotalBtcAmount();
    btcChangeAmount = totalBtcAmount - minimumBtcAmount();
  }

  public static class Output {
    long value;
    Address address;
    byte[] scriptData;

    public Output(long value, Address address, byte[] scriptData) {
      this.value = value;
      this.address = address;
      this.scriptData = scriptData;
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

        if (btcChangeAmount < MIN_NONDUST_OUTPUT.value) {
            throw new ImkeyException(Messages.IMKEY_AMOUNT_LESS_THAN_MINIMUM);
        }

        NetworkParameters networkParameters = null;
        if(Constants.MAINNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_MAINNET);
        } else if (Constants.TESTNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_TESTNET);
        } else {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }

        // select applet
        selectApplet();

        ImkeyTransaction tran = new ImkeyTransaction(networkParameters);

        // get main pubkey
        String mainPubKeyRes = new Btc().getXpubHex(pathPrefix.substring(0,pathPrefix.length()-1), false);
        DeterministicKey mainPubKey = HDKeyDerivation.createMasterPubKeyFromBytes(
                NumericUtil.hexToBytes(mainPubKeyRes.substring(0, 130)),
                NumericUtil.hexToBytes(mainPubKeyRes.substring(130)));

        List<byte[]> pubKeys = new ArrayList<>();

        for (int i = 0; i < getOutputs().size(); i++) {
            UTXO output = getOutputs().get(i);
            DeterministicKey childPubKey = EccUtil.deriveChildKeyFromPublic(mainPubKey, output.getDerivedPath());
            byte[] pubKey = childPubKey.getPubKey();
            // verify address
            if (!verifyAddr(getOutputs().get(i).getAddress(), networkParameters, pubKey)) {
                throw new ImkeyException(Messages.IMKEY_ADDRESS_MISMATCH_WITH_PATH);
            }

            pubKeys.add(pubKey);
        }

        sender = Address.fromBase58(networkParameters, getOutputs().get(0).getAddress());

        tran.addOutput(Coin.valueOf(btcChangeAmount), sender);

        //add send to output
        tran.addOutput(MIN_NONDUST_OUTPUT, Address.fromBase58(networkParameters, getTo()));

        // add data output
        tran.addOutput(Coin.ZERO, createOmniExtraData(getAmount()));

        //output serialize
        String signedHex = NumericUtil.bytesToHex(tran.serializeTransaction(Transaction.SigHash.ALL, false));


        byte[] bytes = ByteUtil.longToByteArray(getFee());
        signedHex += ByteUtil.byteArrayToHexString(bytes);

        // get address version
        String version = Integer.toHexString(Address.fromBase58(networkParameters, getTo()).getVersion());
        if(version.length()%2 !=0) {
            version = "0" + version;
        }
        signedHex += version;

        // prepare
        byte[] signBytes = ByteUtil.hexStringToByteArray(signedHex);
        signBytes[4] = (byte)getOutputs().size();
        byte[] SignBytesWtl = ByteUtil.concat(new byte[]{(byte)0x01}, ByteUtil.concat(new byte[]{(byte)signBytes.length},signBytes));
        byte[] hashData  = Sha256Hash.hashTwice(SignBytesWtl);
        byte[] signature = Wallet.signPackage(Sha256Hash.wrap(hashData));
        byte[] signatureWtl = ByteUtil.concat(new byte[]{(byte)0x00}, ByteUtil.concat(new byte[]{(byte)signature.length},signature));
        byte[] apduPack = ByteUtil.concat(signatureWtl, SignBytesWtl);

        LogUtil.d("btc prepare....");
        String res = Ble.getInstance().sendApdu(Apdu.omniPrepareData((byte)0x00, NumericUtil.bytesToHex(apduPack)), Constants.SEND_SIGN_PRE_APDU_TIMEOUT);
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

            for(int y = i * Constants.EACH_ROUND_NUMBER; y < (i + 1) * Constants.EACH_ROUND_NUMBER; y++){
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

                String path = pathPrefix + output.getDerivedPath();
                String sig = Apdu.btcSign(y, Apdu.Hash_ALL, path);
                String sigRes = Ble.getInstance().sendApdu(sig);
                LogUtil.d("signResult" + y + "：" + sigRes);
                // check sign result
                Apdu.checkResponse(sigRes);

                String r = sigRes.substring(2, 66);
                String s = sigRes.substring(66, 130);


                LogUtil.d("\n********************");
                LogUtil.d(i + " r:" + r);
                LogUtil.d(i + " s:" + s);

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

        NetworkParameters networkParameters = null;
        if(Constants.MAINNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_MAINNET);
        } else if (Constants.TESTNET.equals(network)) {
            networkParameters = NetworkParameters.fromID(NetworkParameters.ID_TESTNET);
        } else {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }

        // select applet
        selectApplet();

        ImkeyTransaction tran = new ImkeyTransaction(MainNetParams.get());

        // get main pubkey
        String mainPubKeyRes = new Btc().getXpubHex(pathPrefix.substring(0,pathPrefix.length()-1), false);
        DeterministicKey mainPubKey = HDKeyDerivation.createMasterPubKeyFromBytes(
                NumericUtil.hexToBytes(mainPubKeyRes.substring(0, 130)),
                NumericUtil.hexToBytes(mainPubKeyRes.substring(130)));

        List<byte[]> pubKeys = new ArrayList<>();

        // get pubkey
        for (int i = 0; i < getOutputs().size(); i++) {
            UTXO output = getOutputs().get(i);
            DeterministicKey childPubKey = EccUtil.deriveChildKeyFromPublic(mainPubKey, output.getDerivedPath());
            byte[] pubKey = childPubKey.getPubKey();

            // verify address
            if(!verifyAddrSegwit(getOutputs().get(i).getAddress(), networkParameters, pubKey)) {
                throw new ImkeyException(Messages.IMKEY_ADDRESS_MISMATCH_WITH_PATH);
            }
            pubKeys.add(pubKey);
        }

        if (btcChangeAmount < MIN_NONDUST_OUTPUT.value) {
            throw new ImkeyException(Messages.IMKEY_AMOUNT_LESS_THAN_MINIMUM);
        }

        long[] inputvalue = new long[getOutputs().size()];

        Address toAddress = Address.fromBase58(networkParameters, getTo());
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
        sender = Address.fromBase58(networkParameters, getOutputs().get(0).getAddress());
        Script changeScriptPubKey = ScriptBuilder.createP2SHOutputScript(sender.getHash160());
        tran.addOutput(Coin.valueOf(btcChangeAmount), changeScriptPubKey);
        // construct output
        tran.addOutput(Coin.valueOf(MIN_NONDUST_OUTPUT.value), targetScriptPubKey);
        // add data output
        this.omniExtraScript = createOmniExtraData(getAmount());
        tran.addOutput(Coin.ZERO, omniExtraScript);

        //serialize transaction data
        int outputSize = tran.getOutputs().size();String signedHexraw = NumericUtil.bytesToHex(tran.serializeSegWitTransaction(Transaction.SigHash.ALL, false, 0, outputSize, inputvalue));
        byte[] bytes = ByteUtil.longToByteArray(getFee());
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
        List<String> pres = Apdu.omniSegwitPrepare((byte)0x00, apduPack);
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
            byte[] scriptCode = NumericUtil.hexToBytes(String.format("0x76a914%s88ac", NumericUtil.bytesToHex(key.getPubKeyHash())));   //@XM don't put length infront
            // create input
            TransactionOutPoint outPoint = new TransactionOutPoint(networkParameters, output.getVout(), Sha256Hash.wrap(output.getTxHash()));
            TransactionInput input = new TransactionInput(networkParameters, null, scriptCode, outPoint, Coin.valueOf(output.getAmount()));
            inputvalue[i] = output.getAmount();
            // add input
            tran.addInput(input);
            try {
                txHashVoutStream.write(NumericUtil.reverseBytes(NumericUtil.hexToBytes(output.getTxHash())));
                Utils.uint32ToByteStreamLE(output.getVout(), txHashVoutStream);
                Utils.uint32ToByteStreamLE(output.getSequence(), sequenceStream);
            } catch (IOException e) {
                throw new ImkeyException("OutputStream error");
            }
        }

        List<String> txHashVoutApduList =  Apdu.omniSegwitPrepare((byte)0x40, txHashVoutStream.toByteArray());
        List<String> sequenceApduList =  Apdu.omniSegwitPrepare((byte)0x80, sequenceStream.toByteArray());
        txHashVoutApduList.addAll(sequenceApduList);
        for(String apdu : txHashVoutApduList){
            Apdu.checkResponse(Ble.getInstance().sendApdu(apdu, Constants.SENT_APDU_TIMEOUT));
        }

        try {

            // calc witnesses and redemScripts
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
                String path = pathPrefix + utxo.getDerivedPath();
                byte[] pathBytesTemp = new byte[path.getBytes().length + 1];
                pathBytesTemp[0] = (byte)path.getBytes().length;
                System.arraycopy(path.getBytes(), 0, pathBytesTemp, 1, path.getBytes().length);
                byte[] data = ByteUtil.concat(tempBytes, pathBytesTemp);

                //sign
                String sigApdu = Apdu.btcSegwitSign(i==getOutputs().size()-1, Apdu.Hash_ALL, data);
                String sigRes = Ble.getInstance().sendApdu(sigApdu);
                Apdu.checkResponse(sigRes);
                LogUtil.d("signResult" + i + "：" + sigRes);
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
                    stream.write(NumericUtil.reverseBytes(NumericUtil.hexToBytes(utxo.getTxHash())));
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
                Utils.uint64ToByteStreamLE(BigInteger.valueOf(btcChangeAmount), stream);
                stream.write(new VarInt(changeScriptPubKey.getProgram().length).encode());
                stream.write(changeScriptPubKey.getProgram());

                Utils.uint64ToByteStreamLE(BigInteger.valueOf(MIN_NONDUST_OUTPUT.value), stream);
                stream.write(new VarInt(targetScriptPubKey.getProgram().length).encode());
                stream.write(targetScriptPubKey.getProgram());

                // write USDT extra data
                Utils.uint64ToByteStreamLE(BigInteger.valueOf(0), stream);
                stream.write(new VarInt(omniExtraScript.getProgram().length).encode());
                stream.write(omniExtraScript.getProgram());

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

                Utils.uint32ToByteStreamLE(getLocktime(), stream);
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

  private long calcTotalBtcAmount() {
    long totalAmount = 0L;

    for (UTXO output : getOutputs()) {
      totalAmount += output.getAmount();
    }

    if (totalAmount < minimumBtcAmount()) {
      throw new ImkeyException(Messages.IMKEY_INSUFFICIENT_FUNDS);
    }
    return totalAmount;
  }

  private long minimumBtcAmount() {
    return MIN_NONDUST_OUTPUT.value + getFee();
  }


  private byte[] hashSequence() throws IOException {
    ByteArrayOutputStream stream = new UnsafeByteArrayOutputStream();

    for (UTXO utxo : getOutputs()) {
      Utils.uint32ToByteStreamLE(utxo.getSequence(), stream);
    }
    return Sha256Hash.hashTwice(stream.toByteArray());
  }

  private Script createOmniExtraData(long amount) {
    ScriptBuilder scriptBuilder = new ScriptBuilder();

    byte[] layerIdBytes = NumericUtil.hexToBytes("0x6f6d6e6900000000");
    byte[] propertyIdBytes = NumericUtil.bigIntegerToBytesWithZeroPadded(BigInteger.valueOf(propertyId), 4);
    byte[] OMNI_DATA_PREFIX = ByteUtil.concat(layerIdBytes, propertyIdBytes);
    byte[] amountBytes = NumericUtil.bigIntegerToBytesWithZeroPadded(BigInteger.valueOf(amount), 8);
    byte[] data = ByteUtil.concat(OMNI_DATA_PREFIX, amountBytes);
    return scriptBuilder
        .op(ScriptOpCodes.OP_RETURN)
        .data(data)
        .build();
  }
}
