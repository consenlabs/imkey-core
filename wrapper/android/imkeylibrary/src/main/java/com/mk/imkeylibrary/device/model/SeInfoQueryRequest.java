package com.mk.imkeylibrary.device.model;


/**
 * se info query
 */
public class SeInfoQueryRequest extends CommonRequest {
    //SE ID
    public String seid;
    //device serial number
    public String sn;
    //SDK version
    public String sdkVersion;

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

    public String getSdkVersion() {
        return sdkVersion;
    }

    public void setSdkVersion(String sdkVersion) {
        this.sdkVersion = sdkVersion;
    }
}
