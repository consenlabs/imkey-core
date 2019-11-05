package com.mk.imkeylibrary.device.model;


public class AppUpdateResponse extends CommonResponse {
    //SE唯一标识
    private String seid;
    private String paid;//应用包AID
    private String instanceAid;//实例AID

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public String getPaid() {
        return paid;
    }

    public void setPaid(String paid) {
        this.paid = paid;
    }

    public String getInstanceAid() {
        return instanceAid;
    }

    public void setInstanceAid(String instanceAid) {
        this.instanceAid = instanceAid;
    }

    @Override
    public String toString() {
        return "AppUpdateResponse{" +
                "seid='" + seid + '\'' +
                ", paid='" + paid + '\'' +
                ", instanceAid='" + instanceAid + '\'' +
                '}';
    }
}
