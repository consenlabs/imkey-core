package com.mk.imkeylibrary.device.model;

import java.sql.Timestamp;

/**
 * SE与应用包关系表
 */
public class SeAppList {

    public String seid;//卡片唯一标识
    public String paid;//应用AID
    public String instanceAid;//实例应用AID
    public String appVersion;//应用包版本号
    public String sdAid;//安全域AID
    public String status;//状态 00:已空发 01：已个人化
    public Timestamp createtime;//创建时间
    public Timestamp updatetime;//更新时间
    public String appletName;

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

    public String getAppVersion() {
        return appVersion;
    }

    public void setAppVersion(String appVersion) {
        this.appVersion = appVersion;
    }

    public String getSdAid() {
        return sdAid;
    }

    public void setSdAid(String sdAid) {
        this.sdAid = sdAid;
    }

    public String getStatus() {
        return status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public Timestamp getCreatetime() {
        return createtime;
    }

    public void setCreatetime(Timestamp createtime) {
        this.createtime = createtime;
    }

    public Timestamp getUpdatetime() {
        return updatetime;
    }

    public void setUpdatetime(Timestamp updatetime) {
        this.updatetime = updatetime;
    }

    public String getAppletName() {
        return appletName;
    }

    public void setAppletName(String appletName) {
        this.appletName = appletName;
    }
}
