package com.mk.imkeylibrary.device.model;


public class SeActivateResponse extends CommonResponse {
    private String seid;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    @Override
    public String toString() {
        return "SeActivateResponse{" +
                "seid='" + seid + '\'' +
                '}';
    }
}
