package com.mk.imkeylibrary.device.model;

/**
 * 设备证书校验反馈
 */
public class DeviceCertCheckResponse extends CommonResponse {
    //SE唯一标识
    public String seid;
    //校验结果
    public boolean verifyResult;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public boolean getVerifyResult() {
        return verifyResult;
    }

    public void setVerifyResult(boolean verifyResult) {
        this.verifyResult = verifyResult;
    }
}
