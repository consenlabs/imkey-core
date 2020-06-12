package com.mk.imkeylibrary.device.model;

import java.util.List;

public class ImKeyDevice {

    //SE唯一标识
    private String seid;
    //设备标识
    private String sn;
    //状态
    private  String status;

    private  String sdkMode;

    public List<AvailableAppInfo> availableAppList;

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

    public String getStatus() {
        return status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public String getSdkMode() {
        return sdkMode;
    }

    public void setSdkMode(String sdkMode) {
        this.sdkMode = sdkMode;
    }

    public List<AvailableAppInfo> getAvailableAppList() {
        return availableAppList;
    }

    public void setAvailableAppList(List<AvailableAppInfo> availableAppList) {
        this.availableAppList = availableAppList;
    }

    @Override
    public String toString() {
        return "ImKeyDevice{" +
                "seid='" + seid + '\'' +
                ", sn='" + sn + '\'' +
                ", status='" + status + '\'' +
                ", sdkMode='" + sdkMode + '\'' +
                ", availableAppList=" + availableAppList +
                '}';
    }
}
