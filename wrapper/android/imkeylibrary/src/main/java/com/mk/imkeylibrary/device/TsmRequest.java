package com.mk.imkeylibrary.device;


import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;

public class TsmRequest {
    protected String getStatus(String response) {
        if (response == null || response.length() == 0) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }
        if (response.length() > 4) {
            return response.substring(response.length() - 4, response.length());
        } else {
            return response;
        }
    }
}
