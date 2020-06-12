package com.mk.imkeylibrary.device.model;

public class AppDeleteRequest extends CommonRequest {
    //SE唯一标识
    public String seid;
    //实例AID
    public String instanceAid;
    //设备公钥证书
    public String deviceCert;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public String getInstanceAid() {
        return instanceAid;
    }

    public void setInstanceAid(String instanceAid) {
        this.instanceAid = instanceAid;
    }

    public String getDeviceCert() {
        return deviceCert;
    }

    public void setDeviceCert(String deviceCert) {
        this.deviceCert = deviceCert;
    }
}
