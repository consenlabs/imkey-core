package com.mk.imkeylibrary.device.key;

import android.content.Context;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;


public class KeyFileManager {

    private static String fileName = "keys";

    public static String getKeysFromLocalFile(String seid) {
        Context context = Ble.getInstance().getContext();
        if(null == context) {
            throw new ImkeyException(Messages.IMKEY_SDK_BLE_NOT_INITIALIZE);
        }

        // file名称与seid关联，支持一个手机，绑定多个设备
        File file = new File(context.getFilesDir(), fileName + seid.substring(seid.length()-8));
        if(!file.exists()) {
            return null;
        }
        String keys;
        FileInputStream fis = null;
        try {
            fis = new FileInputStream(file);
            //获取文件长度
            int length = fis.available();
            byte[] buffer = new byte[length];
            fis.read(buffer);
            keys = new String(buffer);
        } catch (IOException e) {
            throw new ImkeyException(Messages.IMKEY_KEYFILE_IO_ERROR);
        } finally {
            if(null!=fis) {
                try {
                    fis.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
        return keys;
    }

    public static void saveKeysToLocalFile(String keys, String seid) {

        Context context = Ble.getInstance().getContext();
        if(null == context) {
            throw new ImkeyException(Messages.IMKEY_SDK_BLE_NOT_INITIALIZE);
        }
        File file;
        FileOutputStream fos = null;
        try {
            file = new File(context.getFilesDir(), fileName + seid.substring(seid.length()-8));

            if(!file.exists()) {
                if(!file.createNewFile()) {
                    throw new ImkeyException(Messages.IMKEY_KEYFILE_CREATE_ERROR);
                }
            }

            fos = new FileOutputStream(file);
            byte[] bytes = keys.getBytes();
            fos.write(bytes);
        } catch (IOException e) {
            throw new ImkeyException(Messages.IMKEY_KEYFILE_IO_ERROR);
        } finally {
            if(null!=fos) {
                try {
                    fos.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}
