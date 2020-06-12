package com.mk.imkeylibrary.device.model;


public class SeSecureCheckRequest extends CommonRequest {
    //SE唯一标识
    private String seid;
    //设备标识
    private String sn;

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

    @Override
    public String toString() {
        return "SeSecureCheckRequest{" +
                "seid='" + seid + '\'' +
                ", sn='" + sn + '\'' +
                ", stepKey='" + stepKey + '\'' +
                ", statusWord='" + statusWord + '\'' +
                ", cardRetDataList=" + cardRetDataList +
                '}';
    }
}
