package com.mk.imkeylibrary.device;

import android.util.Base64;

import org.json.JSONException;
import org.json.JSONObject;

import java.security.PrivateKey;
import java.security.PublicKey;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Pattern;

import javax.crypto.KeyAgreement;

import com.google.protobuf.Any;
import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.core.Apdu;
import com.mk.imkeylibrary.core.foundation.crypto.AES;
import com.mk.imkeylibrary.core.foundation.crypto.Hash;
import com.mk.imkeylibrary.core.foundation.crypto.RsaCrypto;
import com.mk.imkeylibrary.device.key.KeyConverter;
import com.mk.imkeylibrary.device.key.KeyFileManager;
import com.mk.imkeylibrary.device.key.KeyManager;
import com.mk.imkeylibrary.device.model.AppDeleteRequest;
import com.mk.imkeylibrary.device.model.AppDeleteResponse;
import com.mk.imkeylibrary.device.model.AppDownloadRequest;
import com.mk.imkeylibrary.device.model.AppDownloadResponse;
import com.mk.imkeylibrary.device.model.AppUpdateRequest;
import com.mk.imkeylibrary.device.model.AppUpdateResponse;
import com.mk.imkeylibrary.device.model.AuthCodeStorageRequest;
import com.mk.imkeylibrary.device.model.AuthCodeStorageResponse;
import com.mk.imkeylibrary.device.model.CertificateSCP11;
import com.mk.imkeylibrary.device.model.DeviceCertCheckRequest;
import com.mk.imkeylibrary.device.model.DeviceCertCheckResponse;
import com.mk.imkeylibrary.device.model.ImKeyDevice;
import com.mk.imkeylibrary.device.model.SdkInfo;
import com.mk.imkeylibrary.device.model.SeActivateRequest;
import com.mk.imkeylibrary.device.model.SeActivateResponse;
import com.mk.imkeylibrary.device.model.SeAppList;
import com.mk.imkeylibrary.device.model.SeInfoQueryRequest;
import com.mk.imkeylibrary.device.model.SeInfoQueryResponse;
import com.mk.imkeylibrary.device.model.SeSecureCheckRequest;
import com.mk.imkeylibrary.device.model.SeSecureCheckResponse;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.keycore.Api;
import com.mk.imkeylibrary.keycore.RustApi;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;
import com.mk.imkeylibrary.utils.NumericUtil;

import deviceapi.Device;

public class DeviceManager {

    /**
     * get se id
     *
     * @return
     */
    public String getSeId() {
        /*//select ISD
        Ble.getInstance().sendApdu(Constants.APDU_SELECT_ISD);
        String res = Ble.getInstance().sendApdu(Constants.APDU_GET_SEID);
        Apdu.checkResponse(res);
        return Apdu.getResponseData(res);*/

        /*String seid = RustApi.INSTANCE.get_seid();
        seid = seid.substring(0, seid.length()-4);
        return seid;*/

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_seid")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String seid = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            seid = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }

        return seid;

    }

    public String getSn() {

        /*//select ISD
        Ble.getInstance().sendApdu(Constants.APDU_SELECT_ISD);
        String res = Ble.getInstance().sendApdu(Constants.APDU_GET_SN);
        Apdu.checkResponse(res);
        String snHex = Apdu.getResponseData(res);
        String sn = new String(ByteUtil.hexStringToByteArray(snHex));
        LogUtil.d(sn);
        return sn;*/


        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_sn")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String sn = null;
        try {
            String sn_result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(sn_result));
            sn = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }

        sn = new String(ByteUtil.hexStringToByteArray(sn));
        return sn;


    }

    public int getRamSize() {

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_ram_size")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String res = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            res = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }

        /*String res = Ble.getInstance().sendApdu(Constants.APDU_GET_RAM_SIZE);
        Apdu.checkResponse(res);*/
        String hexSize = res.substring(4,8);
        return Integer.parseInt(hexSize,16);
    }

    /**
     * 获取固件版本
     * @return
     */
    public String getFirmwareVersion() {


        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_firmware_version")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String res = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            res = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }


        /*Ble.getInstance().sendApdu(Constants.APDU_SELECT_ISD);
        String res = Ble.getInstance().sendApdu(Constants.APDU_GET_COS_VERSION);*/
        //Apdu.checkResponse(res);

        String version = Apdu.getResponseData(res);
        StringBuffer sb = new StringBuffer();
        sb.append(version.substring(0, 1));
        sb.append('.');
        sb.append(version.substring(1, 2));
        sb.append('.');
        sb.append(version.substring(2));
        return sb.toString();
    }

    /**
     * get device remaining battery power
     *
     * @return
     */
    public String getBatteryPower() {

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_battery_power")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String res = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            res = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }

        /*Ble.getInstance().sendApdu(Constants.APDU_SELECT_ISD);
        String res = Ble.getInstance().sendApdu(Constants.APDU_GET_BATTERY_POWER);*/
        //Apdu.checkResponse(res);
        //String batteryPower = Apdu.getResponseData(res);

        String batteryPower = res;
        if (!batteryPower.equals(Constants.BATTERY_CHARGING_SIGN)) {
            batteryPower = String.valueOf(Integer.parseInt(batteryPower, 16));
        }

        return batteryPower;
    }

    /**
     * 获取硬件当前生命周期
     * @return
     */
    public String getLifeTime() {

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_life_time")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String res = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            res = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }

        /*String res = Ble.getInstance().sendApdu(Constants.APDU_GET_LIFE_TIME);
        Apdu.checkResponse(res);*/
        //String lifeTime = Apdu.getResponseData(res);
        switch (res) {
            case "80":
                return Constants.LIFE_TIME_DEVICE_INITED;
            case "89":
                return Constants.LIFE_TIME_DEVICE_ACTIVATED;
            case "81":
                return Constants.LIFE_TIME_UNSET_PIN;
            case "83":
                return Constants.LIFE_TIME_WALLET_UNREADY;
            case "84":
                return Constants.LIFE_TIME_WALLET_CREATTING;
            case "85":
                return Constants.LIFE_TIME_WALLET_RECOVERING;
            case "86":
                return Constants.LIFE_TIME_WALLET_READY;
            default:
                return Constants.LIFE_TIME_UNKNOWN;
        }
    }

    public String getBleName() {

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_life_time")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String res = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            res = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }

        /*String result = Ble.getInstance().sendApdu(Constants.APDU_GET_BLE_NAME);*/

        byte[] bytes = ByteUtil.hexStringToByteArray(res);
        res = new String(bytes);
        return res;
    }

    public void setBleName(String bleName) {

        String regEx = "^[a-zA-Z0-9\\-]{1,12}$";
        if(!Pattern.matches(regEx, bleName)) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }
        String apdu = Apdu.setBleName(bleName);
        String result = Ble.getInstance().sendApdu(apdu);
        Apdu.checkResponse(result);
    }

    /**
     * 获取蓝牙版本
     * @return
     */
    public String getBleVersion() {

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("get_ble_version")
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        String res = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            Device.ApduResponse response = Device.ApduResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            res = response.getResult();
        } catch (Exception e) {
            e.printStackTrace();
        }


        /*Ble.getInstance().sendApdu(Constants.APDU_SELECT_ISD);
        String result = Ble.getInstance().sendApdu(Constants.APDU_GET_BLE_VERSION);
        Apdu.checkResponse(result);*/
        String version = res.substring(0, 4);
        String[] chas = version.split("");
        return chas[1] + "." + chas[2] + "." + chas[3] + chas[4];
    }

    private String getCert() {
        Ble.getInstance().sendApdu(Constants.APDU_SELECT_ISD);
        return Ble.getInstance().sendApdu(Constants.APDU_GET_CERT);
    }

    public void checkDevice() {

        String seid = getSeId();
        String sn = getSn();

        deviceapi.Device.SeAction seAction = deviceapi.Device.SeAction.newBuilder()
                .setSeId(seid)
                .setSn(sn)
                .setSdkVersion(Constants.version)
                .build();

        Any any = Any.newBuilder()
                .setValue(seAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("se_secure_check")
                .setParam(any)
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            //Device.SeQueryResponse response = Device.SeQueryResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            //String s = response.toString();
            LogUtil.d(result);
        } catch (Exception e) {
            e.printStackTrace();
        }

        /*String seId = getSeId();
        String sn = getSn();

        // 组织请求参数
        SeSecureCheckRequest request = new SeSecureCheckRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setSn(sn);
        request.setDeviceCert(getCert());
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        SeSecureCheckResponse response = new SeCheck().checkSe(request);

        // 判断处理状态
        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            if (Constants.TSM_RETURNCODE_DEVICE_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_AUTHENTICITY_CHECK_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEV_INACTIVATED.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_NOT_ACTIVATED);
            } else if (Constants.TSM_RETURNCODE_DEVICE_ILLEGAL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
            } else if (Constants.TSM_RETURNCODE_OCE_CERT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_OCE_CERT_CHECK_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
            } else if (Constants.TSM_RETURNCODE_RECEIPT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_RECEIPT_CHECK_FAIL);
            } else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
            }
        }*/
    }


    public void activeDevice() {

        String seid = getSeId();
        String sn = getSn();

        deviceapi.Device.SeAction seAction = deviceapi.Device.SeAction.newBuilder()
                .setSeId(seid)
                .setSn(sn)
                .setSdkVersion(Constants.version)
                .build();

        Any any = Any.newBuilder()
                .setValue(seAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("se_activate")
                .setParam(any)
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            //Device.SeQueryResponse response = Device.SeQueryResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            //String s = response.toString();
            LogUtil.d(result);
        } catch (Exception e) {
            e.printStackTrace();
        }

        /*
        String seId = getSeId();
        String sn = getSn();

        SeActivateRequest request = new SeActivateRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setSn(sn);
        request.setStatusWord("");
        request.setDeviceCert(getCert());
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        SeActivateResponse response = new SeActive().activeSe(request);

        // 判断处理状态
        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            if (Constants.TSM_RETURNCODE_DEVICE_ACTIVE_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ACTIVE_FAIL);
            } else if (Constants.TSM_RETURNCODE_SEID_ILLEGAL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
            } else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
            }
        }*/

    }

    public String checkUpdate() {

        String seid = getSeId();
        String sn = getSn();

        deviceapi.Device.SeAction seAction = deviceapi.Device.SeAction.newBuilder()
                .setSeId(seid)
                .setSn(sn)
                .setSdkVersion(Constants.version)
                .build();

        Any any = Any.newBuilder()
                .setValue(seAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("se_query")
                .setParam(any)
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        Device.SeQueryServiceResponse response = null;
        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            response = Device.SeQueryServiceResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
        } catch (Exception e) {
            e.printStackTrace();
        }
        return response.toString();

        /*ImKeyDevice imKeyDevice = new ImKeyDevice();
        String seId = getSeId();
        String sn = getSn();

        imKeyDevice.setSeid(seId);
        imKeyDevice.setSn(sn);

        SeQuery seQuery = new SeQuery();
        SeInfoQueryRequest request = new SeInfoQueryRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setSn(sn);
        request.setSdkVersion(Constants.version);

        SeInfoQueryResponse response = seQuery.query(request);

        String returnCode = response.get_ReturnCode();
        if (Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            imKeyDevice.setStatus(Constants.IMKEY_DEV_STATUS_LATEST);
            imKeyDevice.setSdkMode(response.getSdkMode());
            imKeyDevice.setAvailableAppList(response.getAvailableAppList());

        } else if (Constants.TSM_RETURNCODE_DEV_INACTIVATED.equals(returnCode)) {
            imKeyDevice.setStatus(Constants.IMKEY_DEV_STATUS_INACTIVATED);
            imKeyDevice.setSdkMode(response.getSdkMode());
            imKeyDevice.setAvailableAppList(response.getAvailableAppList());

        } else if (Constants.TSM_RETURNCODE_DEVICE_ILLEGAL.equals(returnCode)) {
            throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
        } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
            throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
        } else if (Constants.TSM_RETURNCODE_SE_QUERY_FAIL.equals(returnCode)) {
            throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_UPDATE_CHECK_FAIL);
        } else {
            throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
        }
        return imKeyDevice;*/
    }

    public void download(String appletName) {

        String seid = getSeId();
        String sn = getSn();

        deviceapi.Device.SeAction seAction = deviceapi.Device.SeAction.newBuilder()
                .setSeId(seid)
                .setSn(sn)
                .setSdkVersion(Constants.version)
                .build();

        Any any = Any.newBuilder()
                .setValue(seAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("app_download")
                .setParam(any)
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            //Device.SeQueryResponse response = Device.SeQueryResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            //String s = response.toString();
            LogUtil.d(result);
        } catch (Exception e) {
            e.printStackTrace();
        }
        /*
        String seId = getSeId();

        AppDownloadRequest request = new AppDownloadRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setInstanceAid(Applet.appletName2instanceAid(appletName));
        request.setDeviceCert(getCert());
        request.setStatusWord("");
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        AppDownloadResponse response = new AppDownload().download(request);

        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            if (Constants.TSM_RETURNCODE_APP_DOWNLOAD_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_APP_DOWNLOAD_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_ILLEGAL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
            } else if (Constants.TSM_RETURNCODE_OCE_CERT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_OCE_CERT_CHECK_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
            } else if (Constants.TSM_RETURNCODE_RECEIPT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_RECEIPT_CHECK_FAIL);
            } else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
            }
        }*/

    }

    public void update(String appletName) {

        String seid = getSeId();
        String sn = getSn();

        deviceapi.Device.SeAction seAction = deviceapi.Device.SeAction.newBuilder()
                .setSeId(seid)
                .setSn(sn)
                .setSdkVersion(Constants.version)
                .build();

        Any any = Any.newBuilder()
                .setValue(seAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("app_update")
                .setParam(any)
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            //Device.SeQueryResponse response = Device.SeQueryResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            //String s = response.toString();
            LogUtil.d(result);
        } catch (Exception e) {
            e.printStackTrace();
        }
        /*
        String seId = getSeId();

        AppUpdateRequest request = new AppUpdateRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setInstanceAid(Applet.appletName2instanceAid(appletName));
        request.setDeviceCert(getCert());
        request.setStatusWord("");
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        AppUpdateResponse response = new AppUpdate().update(request);

        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            if (Constants.TSM_RETURNCODE_APP_UPDATE_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_APP_UPDATE_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_ILLEGAL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
            } else if (Constants.TSM_RETURNCODE_OCE_CERT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_OCE_CERT_CHECK_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
            } else if (Constants.TSM_RETURNCODE_RECEIPT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_RECEIPT_CHECK_FAIL);
            } else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
            }
        }*/

    }

    public void delete(String appletName) {

        String seid = getSeId();
        String sn = getSn();

        deviceapi.Device.SeAction seAction = deviceapi.Device.SeAction.newBuilder()
                .setSeId(seid)
                .setSn(sn)
                .setSdkVersion(Constants.version)
                .build();

        Any any = Any.newBuilder()
                .setValue(seAction.toByteString())
                .build();

        api.Api.DeviceParam deviceParam = api.Api.DeviceParam.newBuilder()
                .setAction("app_delete")
                .setParam(any)
                .build();

        Any any2 = Any.newBuilder()
                .setValue(deviceParam.toByteString())
                .build();

        api.Api.TcxAction action = api.Api.TcxAction.newBuilder()
                .setMethod("device_manage")
                .setParam(any2)
                .build();
        String hex = NumericUtil.bytesToHex(action.toByteArray());

        try {
            String result = RustApi.INSTANCE.call_tcx_api(hex);
            //Device.SeQueryResponse response = Device.SeQueryResponse.parseFrom(ByteUtil.hexStringToByteArray(result));
            //String s = response.toString();
            LogUtil.d(result);
        } catch (Exception e) {
            e.printStackTrace();
        }
        /*
        String seId = getSeId();

        AppDeleteRequest request = new AppDeleteRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setInstanceAid(Applet.appletName2instanceAid(appletName));
        request.setDeviceCert(getCert());
        request.setStatusWord("");
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        AppDeleteResponse response = new AppDelete().delete(request);

        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            if (Constants.TSM_RETURNCODE_APP_DELETE_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_APP_DELETE_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_ILLEGAL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
            } else if (Constants.TSM_RETURNCODE_OCE_CERT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_OCE_CERT_CHECK_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
            } else if (Constants.TSM_RETURNCODE_RECEIPT_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_RECEIPT_CHECK_FAIL);
            } else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
            }
        }*/

    }


    private void deviceCertCheck(String seId, String sn, String deviceCert) {

        // 组织请求参数
        DeviceCertCheckRequest request = new DeviceCertCheckRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setSn(sn);
        request.setDeviceCert(deviceCert);
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        DeviceCertCheckResponse response = new DeviceCertCheck().deviceCertCheck(request);

        // 判断处理状态
        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            if (Constants.TSM_RETURNCODE_DEVICE_CHECK_FAIL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_AUTHENTICITY_CHECK_FAIL);
            } else if (Constants.TSM_RETURNCODE_DEV_INACTIVATED.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_NOT_ACTIVATED);
            } else if (Constants.TSM_RETURNCODE_DEVICE_ILLEGAL.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_ILLEGAL);
            } else if (Constants.TSM_RETURNCODE_DEVICE_STOP_USING.equals(returnCode)) {
                throw new ImkeyException(Messages.IMKEY_TSM_DEVICE_STOP_USING);
            }  else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
            }
        }

        Boolean verifyResult = response.getVerifyResult();
        if(!verifyResult) {
            throw new ImkeyException(Messages.IMKEY_SE_CERT_INVALID);
        }

    }


    private void authCodeStorage(String authcode) {

        String seId = getSeId();

        // 组织请求参数
        AuthCodeStorageRequest request = new AuthCodeStorageRequest();
        request.setStepKey("01");
        request.setSeid(seId);
        request.setAuthCode(authcode);
        List<String> list = new ArrayList<>();
        request.setCardRetDataList(list);

        AuthCodeStorageResponse response = new AuthCodeStorage().authCodeStorage(request);

        // 判断处理状态
        String returnCode = response.get_ReturnCode();
        if (!Constants.TSM_RETURNCODE_SUCCESS.equals(returnCode)) {
            throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + returnCode);
        }
    }

    /**
     * 设备绑定状态核查
     * @return
     */
    public String bindCheck() {

        //deviceapi.Device.BindCheck bindCheck = new deviceapi.Device.BindCheck().;
        return  "";


        /*// 获取seid
        String seid = getSeId();
        // 获取sn
        String sn = getSn();

        // 计算文件加密密钥
        KeyManager.getInstance().genEncryKey(seid, sn);

        // 获取本地密钥文件内容
        String keys = KeyFileManager.getKeysFromLocalFile(seid);
        Boolean newKeyFlag = false;

        if(null != keys) {
            // 格式  app私钥+app公钥+SE公钥+sessionkey+checksum
            // 解密文件，验证文件内容是否正确，如果不正确，重新生成公私钥
            newKeyFlag = !KeyManager.getInstance().decryptKeys(keys);

        }

        if(null == keys || newKeyFlag) {
            // 生成本地公私钥
            KeyManager.getInstance().genLocalKeys();
            newKeyFlag = true;
        }

        // 发送指令
        // select applet
        selectApplet();
        //获取本地公钥
        byte[] pubKey = KeyManager.getInstance().getPubKey();
        // 调用bindcheck指令
        String bindCheckApdu = Apdu.bindCheck(pubKey);
        String res = Ble.getInstance().sendApdu(bindCheckApdu);
        Apdu.checkResponse(res);

        //状态 0x00: 未绑定  0x55: 与传入appPK绑定  0xAA：与其他appPK绑定
        String status = res.substring(0,2);
        String sePkCert = res.substring(2,res.length()-4);
        ////状态 0x00: 未绑定  0x55: 与传入appPK绑定  0xAA：与其他appPK绑定
        if(status.equals(Constants.BIND_STATUS_UNBOUND) || status.equals(Constants.BIND_STATUS_BOUND_OTHER)) {

            // 验证sePkCertf
            deviceCertCheck(seid, sn, sePkCert);

            // 解析sePkCert
            try {
                byte[] dataBF21 = ByteUtil.hexStringToByteArray(sePkCert);
                CertificateSCP11 certificateSCP11 = new CertificateSCP11(dataBF21, (short)0, (short)(dataBF21.length & 0xFFFF));
                byte[] pubk = certificateSCP11.getSubTLVValue((short) 0x7F49);;
                byte[] sepk = new byte[65];
                System.arraycopy(pubk, 2, sepk, 0, 65);

                KeyManager.getInstance().setSePubKey(sepk);

            } catch (Exception e) {
               throw new ImkeyException(Messages.IMKEY_SE_CERT_INVALID);
            }

            //协商sessionKey
            try {
                PrivateKey privateKey = KeyConverter.getPrivKey(KeyManager.getInstance().getPrivKey());

                PublicKey publicKey = KeyConverter.getPubKey(KeyManager.getInstance().getSePubKey());
                KeyAgreement ka = KeyAgreement.getInstance("ECDH", "SC");
                ka.init(privateKey);
                ka.doPhase(publicKey, true);

                byte[] sessionKey = ka.generateSecret();
                sessionKey = Hash.sha1(sessionKey);
                byte[] aesSessionKey = new byte[16];
                System.arraycopy(sessionKey, 0, aesSessionKey, 0, 16);
                KeyManager.getInstance().setSessionKey(aesSessionKey);

            } catch (Throwable e) {
                throw new ImkeyException(Messages.IMKEY_NEGOTIATE_SESSIONKEY_ERROR);
            }

            // 保存密钥到本地文件
            if(newKeyFlag) {
                String cipherKeys = KeyManager.getInstance().encryptKeys();
                KeyFileManager.saveKeysToLocalFile(cipherKeys, seid);
            }

        }

        // 返回状态
        return Constants.bindcheckStatusMap.get(status);*/

    }

    /**
         * 显示绑定码
     * @return
     */
    public void displayBindingCode() {
        //  产生随机绑定码
        String generateAuthCodeApdu = Apdu.generateAuthCode();
        String res = Ble.getInstance().sendApdu(generateAuthCodeApdu);
        Apdu.checkResponse(res);
    }


    public String bindAcquire(String bindingCode) {
        bindingCode = bindingCode.toUpperCase();
        // authCode 校验  0,1,I,O,排除
        String regEx = "^[A-HJ-NP-Z2-9]{8}$";
        if(!Pattern.matches(regEx, bindingCode)) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }
        // 保存authCode，先存储再验证
        String authCodeCipher = null;
        try {
            authCodeCipher = ByteUtil.byteArrayToHexString(RsaCrypto.encryptByPublicKeyWithPadding(bindingCode.getBytes(), Base64.decode(Constants.AUTHCODE_ENC_PUB_KEY, Base64.DEFAULT)));
        } catch(Exception e) {
            throw new ImkeyException(Messages.IMKEY_ENCRYPT_AUTHCODE_FAIL);
        }
        authCodeStorage(authCodeCipher);

        // select applet
        selectApplet();
        byte[] data = new byte[8 + 65 + 65]; // 授权码、apppk、sepk
        System.arraycopy(bindingCode.getBytes(), 0, data, 0, 8);
        System.arraycopy(KeyManager.getInstance().getPubKey(), 0, data, 8, 65);
        System.arraycopy(KeyManager.getInstance().getSePubKey(), 0, data, 73, 65);
        //计算hash
        byte[] hash = Hash.sha256(data);
        //加密hash
        byte[] cipher = AES.encryptByCBC(hash, KeyManager.getInstance().getSessionKey(), genIV(bindingCode));

        data = new byte[KeyManager.getInstance().getPubKey().length + cipher.length];
        System.arraycopy(KeyManager.getInstance().getPubKey(), 0, data, 0, KeyManager.getInstance().getPubKey().length);
        System.arraycopy(cipher, 0, data, KeyManager.getInstance().getPubKey().length, cipher.length);

        String identityVerifyApdu = Apdu.identityVerify((byte)0x80, data);
        String res = Ble.getInstance().sendApdu(identityVerifyApdu);
        Apdu.checkResponse(res);

        String status = res.substring(0,2);
        // 返回状态
        return Constants.identityVerifyStatusMap.get(status);
    }


    protected void selectApplet() {
        String selectApdu = Apdu.select(Applet.IMK_AID);
        String res = Ble.getInstance().sendApdu(selectApdu);
        Apdu.checkResponse(res);
    }

    private byte[] genIV(String bindingCode) {
        String salt = "bindingCode";
        byte[] bindingCodeHash = Hash.sha256(bindingCode.getBytes());
        byte[] saltHash = Hash.sha256(salt.getBytes());
        for(int i=0; i<bindingCodeHash.length; i++) {
            bindingCodeHash[i] = (byte)(bindingCodeHash[i] ^ saltHash[i]);
        }
        byte[] iv = new byte[16];
        // 取前16个字节作为加密密钥
        System.arraycopy(bindingCodeHash, 0, iv, 0, 16);
        return iv;
    }

    public SdkInfo getSdkInfo() {
        SdkInfo sdkInfo = new SdkInfo();
        sdkInfo.setSdkVersion(Constants.version);
        return sdkInfo;
    }

}
