package com.mk.imkeylibrary.bluetooth.Callback;

import com.mk.imkeylibrary.bluetooth.BleDevice;
import com.mk.imkeylibrary.bluetooth.ErrorCode;

public interface ConnectCallback {
    void onConnecting(BleDevice bleDevice);

    void onConnected(BleDevice bleDevice);

    void onDisconnected(BleDevice bleDevice);

    void onConnectFail(ErrorCode errorCode);
}
