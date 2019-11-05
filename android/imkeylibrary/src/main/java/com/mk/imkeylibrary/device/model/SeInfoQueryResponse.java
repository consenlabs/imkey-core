package com.mk.imkeylibrary.device.model;

import java.util.List;

public class SeInfoQueryResponse extends CommonResponse {
    //SE ID
    public String seid;

    public String sdkMode;

    public List<AvailableAppInfo> availableAppList;

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public List<AvailableAppInfo> getAvailableAppList() {
        return availableAppList;
    }

    public void setAvailableAppList(List<AvailableAppInfo> availableAppList) {
        this.availableAppList = availableAppList;
    }

    public String getSdkMode() {
        return sdkMode;
    }

    public void setSdkMode(String sdkMode) {
        this.sdkMode = sdkMode;
    }

    public static class AppUpdateInfo{
        private String appletName;
        private String appletVersion;
        private String updateType;

        public String getAppletName() {
            return appletName;
        }

        public void setAppletName(String appletName) {
            this.appletName = appletName;
        }

        public String getAppletVersion() {
            return appletVersion;
        }

        public void setAppletVersion(String appletVersion) {
            this.appletVersion = appletVersion;
        }

        public String getUpdateType() {
            return updateType;
        }

        public void setUpdateType(String updateType) {
            this.updateType = updateType;
        }
    }

}
