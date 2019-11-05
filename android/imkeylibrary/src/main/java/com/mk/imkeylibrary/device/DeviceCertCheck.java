package com.mk.imkeylibrary.device;


import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.device.model.CommonResponse;
import com.mk.imkeylibrary.device.model.DeviceCertCheckRequest;
import com.mk.imkeylibrary.device.model.DeviceCertCheckResponse;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.net.Https;
import com.mk.imkeylibrary.utils.LogUtil;

public class DeviceCertCheck extends TsmRequest {
    private static final String ACTION = "deviceCertCheck";

    public DeviceCertCheckResponse deviceCertCheck(DeviceCertCheckRequest request) {
        if (request == null)
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);

        request.setCommandID(ACTION);// set commandId

        String json = toJson(request);
        if (json == null || json.length() == 0) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }

        String res = Https.post(ACTION, json);
        DeviceCertCheckResponse response = fromJson(res);
        if (response == null || response.get_ReturnCode() == null) {
            throw new ImkeyException(Messages.IMKEY_SDK_JSON_PARSE_ERROR);
        }
        return response;
    }

    private String toJson(DeviceCertCheckRequest request) {
        JSONObject jsonObject = new JSONObject();
        try {
            jsonObject.put("seid", request.getSeid());
            jsonObject.put("sn", request.getSn());
            jsonObject.put("stepKey", request.getStepKey());
            jsonObject.put("deviceCert", request.getDeviceCert());
            JSONArray jsonArray = new JSONArray();
            if (request.getCardRetDataList() != null) {
                for (String cardData : request.getCardRetDataList()) {
                    jsonArray.put(cardData);
                }
            }
            jsonObject.put("cardRetDataList", jsonArray);
        } catch (JSONException e) {
            LogUtil.d(e.getMessage());
            return "";
        }
        return jsonObject.toString();
    }

    private DeviceCertCheckResponse fromJson(String json) {
        DeviceCertCheckResponse response = new DeviceCertCheckResponse();
        try {
            JSONObject jsonObject = new JSONObject(json);
            response.set_ReturnCode(jsonObject.getString("_ReturnCode"));
            response.set_ReturnMsg(jsonObject.getString("_ReturnMsg"));
            if (response.get_ReturnCode().equals("000000")) {
                JSONObject returnJsonObj = jsonObject.getJSONObject("_ReturnData");
                CommonResponse.ReturnDataBean returnDataBean = response.new ReturnDataBean();
                returnDataBean.setSeid(returnJsonObj.getString("seid"));
                response.setVerifyResult(Boolean.valueOf(returnJsonObj.getString("verifyResult")));
                response.set_ReturnData(returnDataBean);
            }
        } catch (JSONException e) {
            LogUtil.d(e.getMessage());
            response = null;
        }
        return response;
    }
}
