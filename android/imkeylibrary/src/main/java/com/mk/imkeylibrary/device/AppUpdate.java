package com.mk.imkeylibrary.device;


import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.List;

import com.mk.imkeylibrary.bluetooth.Ble;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.device.model.AppUpdateRequest;
import com.mk.imkeylibrary.device.model.AppUpdateResponse;
import com.mk.imkeylibrary.device.model.CommonResponse;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.net.Https;
import com.mk.imkeylibrary.utils.LogUtil;

public class AppUpdate extends TsmRequest {
    private static final String ACTION = "appUpdate";

    public AppUpdateResponse update(AppUpdateRequest request) {
        if (request == null)
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);

        request.setCommandID(ACTION);// set commandId

        String json = toJson(request);
        if (json == null || json.length() == 0) {
            throw new ImkeyException(Messages.IMKEY_SDK_ILLEGAL_ARGUMENT);
        }

        String res = Https.post(ACTION, json);
        AppUpdateResponse response = fromJson(res);
        if (response == null || response.get_ReturnCode() == null) {
            throw new ImkeyException(Messages.IMKEY_SDK_JSON_PARSE_ERROR);
        }
        if (response.get_ReturnCode().equals("000000")) {
            CommonResponse.ReturnDataBean returnDataBean = response.get_ReturnData();
            if (returnDataBean != null) {
                if ("end".equals(returnDataBean.getNextStepKey())) {
                    return response;
                } else {
                    AppUpdateRequest reRequest = new AppUpdateRequest();
                    reRequest.setStepKey(returnDataBean.getNextStepKey());
                    List<String> apdus = new ArrayList<>();
                    if (returnDataBean.getApduList() != null) {
                        for (int i = 0; i < returnDataBean.getApduList().size(); i++) {
                            String apduRes = Ble.getInstance().sendApdu(returnDataBean.getApduList().get(i));
                            apdus.add(apduRes.toUpperCase());
                            String status = getStatus(apduRes);

                            //check dwonload apdu
                            if("03".equals(returnDataBean.getNextStepKey()) && i>0 && !status.equals("9000")){
                                break;
                            }

                            if (i == returnDataBean.getApduList().size() - 1) {
                                reRequest.setStatusWord(status);
                            }
                        }
                        reRequest.setCardRetDataList(apdus);
                    }
                    reRequest.setSeid(request.getSeid());
                    reRequest.setInstanceAid(request.getInstanceAid());
                    return update(reRequest);
                }
            }
        }
        return response;
    }

    private String toJson(AppUpdateRequest request) {
        JSONObject jsonObject = new JSONObject();
        try {
            jsonObject.put("seid", request.getSeid());
            jsonObject.put("instanceAid", request.getInstanceAid());
            jsonObject.put("stepKey", request.getStepKey());
            jsonObject.put("statusWord", request.getStatusWord());
            jsonObject.put("deviceCert", request.getDeviceCert());
            jsonObject.put("commandID", request.getCommandID());
            JSONArray jsonArray = new JSONArray();
            if (request.getCardRetDataList() != null) {
                for (String cardData : request.getCardRetDataList()) {
                    jsonArray.put(cardData);
                }
            }
            jsonObject.put("cardRetDataList", jsonArray);
        } catch (JSONException e) {
            LogUtil.d(e.getMessage());
            return "";        }
        return jsonObject.toString();
    }

    private AppUpdateResponse fromJson(String json) {
        AppUpdateResponse response = new AppUpdateResponse();
        try {
            JSONObject jsonObject = new JSONObject(json);
            response.set_ReturnCode(jsonObject.getString("_ReturnCode"));
            response.set_ReturnMsg(jsonObject.getString("_ReturnMsg"));
            if (response.get_ReturnCode().equals("000000")) {
                JSONObject returnJsonObj = jsonObject.getJSONObject("_ReturnData");
                CommonResponse.ReturnDataBean returnDataBean = response.new ReturnDataBean();
                returnDataBean.setSeid(returnJsonObj.getString("seid"));
                returnDataBean.setNextStepKey(returnJsonObj.getString("nextStepKey"));
                if (!returnDataBean.getNextStepKey().equals("end")) {
                    JSONArray jsonArray = returnJsonObj.getJSONArray("apduList");
                    List<String> list = new ArrayList<>();
                    for (int i = 0; i < jsonArray.length(); i++) {
                        String temp = jsonArray.getString(i);
                        list.add(temp);
                    }
                    returnDataBean.setApduList(list);
                }
                response.set_ReturnData(returnDataBean);
            }
        } catch (JSONException e) {
            LogUtil.d(e.getMessage());
            response = null;
        }
        return response;
    }
}
