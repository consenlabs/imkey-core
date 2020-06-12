package com.mk.imkeylibrary.device.model;

/**
 * 设备证书校验反馈
 */
public class AuthCodeStorageResponse extends CommonResponse {
    //SE唯一标识
    public String seid;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

}
