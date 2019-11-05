package com.mk.imkeylibrary.device.model;

public class AvailableAppInfo {

    public String appletName;
    public String appLogo;
    public String lastUpated;
    public String installMode;
    public String latestVersion;
    public String installedVersion;

    public String getAppletName() {
        return appletName;
    }

    public void setAppletName(String appletName) {
        this.appletName = appletName;
    }

    public String getAppLogo() {
        return appLogo;
    }

    public void setAppLogo(String appLogo) {
        this.appLogo = appLogo;
    }

    public String getLastUpated() {
        return lastUpated;
    }

    public void setLastUpated(String lastUpated) {
        this.lastUpated = lastUpated;
    }

    public String getInstallMode() {
        return installMode;
    }

    public void setInstallMode(String installMode) {
        this.installMode = installMode;
    }

    public String getLatestVersion() {
        return latestVersion;
    }

    public void setLatestVersion(String latestVersion) {
        this.latestVersion = latestVersion;
    }

    public String getInstalledVersion() {
        return installedVersion;
    }

    public void setInstalledVersion(String installedVersion) {
        this.installedVersion = installedVersion;
    }

    @Override
    public String toString() {
        return "AvailableAppInfo{" +
                "appletName='" + appletName + '\'' +
                ", appLogo='" + appLogo + '\'' +
                ", lastUpated='" + lastUpated + '\'' +
                ", installMode='" + installMode + '\'' +
                ", latestVersion='" + latestVersion + '\'' +
                ", installedVersion='" + installedVersion + '\'' +
                '}';
    }
}
