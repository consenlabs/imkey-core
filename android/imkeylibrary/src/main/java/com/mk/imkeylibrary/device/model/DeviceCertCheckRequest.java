package com.mk.imkeylibrary.device.model;

/**
 * 设备证书校验请求
 */
public class DeviceCertCheckRequest extends CommonRequest {
    //SE唯一标识
    public String seid;
    //设备标识
    public String sn;
    //设备公钥证书
    public String deviceCert;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public String getSn() {
        return sn;
    }

    public void setSn(String sn) {
        this.sn = sn;
    }

    public String getDeviceCert() {
        return deviceCert;
    }

    public void setDeviceCert(String deviceCert) {
        this.deviceCert = deviceCert;
    }
}
