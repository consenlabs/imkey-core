package com.mk.imkeylibrary.device.model;



import java.util.List;


/**
 * 通用请求对象要素
 *
 * @author Administrator
 */
public class CommonRequest {

    String stepKey;//步骤KEY

    String statusWord;//状态字

    List<String> cardRetDataList;//卡片返回数据集合

    String commandID;

    //设备证书
    String deviceCert;

    public String getStepKey() {
        return stepKey;
    }

    public void setStepKey(String stepKey) {
        this.stepKey = stepKey;
    }

    public String getStatusWord() {
        return statusWord;
    }

    public void setStatusWord(String statusWord) {
        this.statusWord = statusWord;
    }

    public String getCommandID() {
        return commandID;
    }
    public void setCommandID(String commandID) {
        this.commandID = commandID;
    }

    public List<String> getCardRetDataList() {
        return cardRetDataList;
    }

    public void setCardRetDataList(List<String> cardRetDataList) {
        this.cardRetDataList = cardRetDataList;
    }

    public String getDeviceCert() {
        return deviceCert;
    }

    public void setDeviceCert(String deviceCert) {
        this.deviceCert = deviceCert;
    }

    @Override
    public String toString() {
        return "CommonRequest{" +
                "stepKey='" + stepKey + '\'' +
                ", statusWord='" + statusWord + '\'' +
                ", cardRetDataList=" + cardRetDataList +
                ", deviceCert='" + deviceCert + '\'' +
                '}';
    }
}
