package com.mk.imkeylibrary.device.model;

public class SeSecureCheckResponse extends CommonResponse {
    //SE唯一标识
    private String seid;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    @Override
    public String toString() {
        return "SeSecureCheckResponse{" +
                "seid='" + seid + '\'' +
                '}';
    }
}
