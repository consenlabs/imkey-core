package com.mk.imkeylibrary.device.model;

/**
 * 设备授权码存储请求
 */
public class AuthCodeStorageRequest extends CommonRequest {
    //SE唯一标识
    public String seid;
    //授权码密文
    public String authCode;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public String getAuthCode() {
        return authCode;
    }

    public void setAuthCode(String authCode) {
        this.authCode = authCode;
    }
}
