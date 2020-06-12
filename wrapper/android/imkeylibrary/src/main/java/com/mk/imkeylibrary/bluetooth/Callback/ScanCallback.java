package com.mk.imkeylibrary.bluetooth.Callback;

import com.mk.imkeylibrary.bluetooth.BleDevice;
import com.mk.imkeylibrary.bluetooth.ErrorCode;

public interface ScanCallback{
    void onScanStarted();

    void onScanDevice(BleDevice bleDevice);

    void onScanStopped();

    void onScanFail(ErrorCode errorCode);
}
