package com.mk.imkeylibrary.device.model;


public class AppDeleteResponse extends CommonResponse {
    //SE唯一标识
    public String seid;
    public String paid;//应用包AID
    public String instanceAid;//实例AID

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
}
