package com.mk.imkeylibrary.net;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import java.net.HttpURLConnection;
import java.net.URL;
import java.security.cert.Certificate;

import javax.net.ssl.HttpsURLConnection;

import com.mk.imkeylibrary.common.Constants;
import com.mk.imkeylibrary.common.Messages;
import com.mk.imkeylibrary.exception.ImkeyException;
import com.mk.imkeylibrary.utils.ByteUtil;
import com.mk.imkeylibrary.utils.LogUtil;

public class Https {

    private static final String LOG_TAG = "imkey";

    public static String post(String action, String json) {
        String url = Constants.HOST_HTTPS + action;
        log("http >>>>>> " + url);
        log("http >>>>>> " + json);
        HttpsURLConnection conn = null;
        BufferedReader br = null;
        try {
            URL objUrl = new URL(url);
            conn = (HttpsURLConnection) objUrl.openConnection();
            conn.setDoOutput(true);
            conn.setDoInput(true);
            conn.setRequestProperty("Content-Type", "application/json");
            conn.setRequestProperty("Accept", "application/json");
            conn.setRequestMethod("POST");
            conn.setConnectTimeout(Constants.HTTP_TIMEOUT);
            conn.setReadTimeout(Constants.HTTP_TIMEOUT);
            OutputStream out = conn.getOutputStream();

            // 获取服务器证书
            Certificate serverCert = conn.getServerCertificates()[0];
            String pubkey = ByteUtil.byteArrayToHexString(serverCert.getPublicKey().getEncoded());
            if(!pubkey.equals(Constants.SSL_CERT_PUBKEY)) {
                throw new ImkeyException(Messages.IMKEY_TSM_SSL_CERT_INVALID);
            }
            OutputStreamWriter wr = new OutputStreamWriter(out);
            wr.write(json);
            wr.close();

            StringBuilder sb = new StringBuilder();
            if (conn.getResponseCode() == HttpURLConnection.HTTP_OK) {
                br = new BufferedReader(new InputStreamReader(conn.getInputStream(), "utf-8"));
                String line;
                while ((line = br.readLine()) != null) {
                    sb.append(line);
                }
                br.close();
                log("http <<<<<< " + sb.toString());
                return sb.toString();
            } else {
                throw new ImkeyException(Messages.IMKEY_TSM_SERVER_ERROR + "_" + conn.getResponseCode());
            }

        }catch (ImkeyException imkeyException){
            throw  imkeyException;
        } catch (Exception exception) {
            // 证书验证异常、网络连接异常等
            throw new ImkeyException(Messages.IMKEY_TSM_NETWORK_ERROR);
        } finally {
            try {
                if (br != null)
                    br.close();
            } catch (IOException e) {
                e.printStackTrace();
            }
            if (conn != null)
                conn.disconnect();
        }
    }

    private static void log(String msg){
        LogUtil.d(LOG_TAG,msg);
    }

}
