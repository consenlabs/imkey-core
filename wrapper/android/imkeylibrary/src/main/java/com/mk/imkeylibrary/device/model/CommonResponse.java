package com.mk.imkeylibrary.device.model;

import java.util.List;


/**
 * 通用反馈报文要素
 * @author Administrator
 *
 */
public class CommonResponse {

	/**
	 * _ReturnCode : 000000
	 * _ReturnMsg : 操作成功
	 * _ReturnData : {"seid":"18000001010000000016","nextStepKey":"02","apduList":["00A4040008A00000000300000000","80500000085D2EAC17C525B4EE00"]}
	 */

	private String _ReturnCode;
	private String _ReturnMsg;
	private ReturnDataBean _ReturnData;

	public String get_ReturnCode() {
		return _ReturnCode;
	}

	public void set_ReturnCode(String _ReturnCode) {
		this._ReturnCode = _ReturnCode;
	}

	public String get_ReturnMsg() {
		return _ReturnMsg;
	}

	public void set_ReturnMsg(String _ReturnMsg) {
		this._ReturnMsg = _ReturnMsg;
	}

	public ReturnDataBean get_ReturnData() {
		return _ReturnData;
	}

	public void set_ReturnData(ReturnDataBean _ReturnData) {
		this._ReturnData = _ReturnData;
	}

	public class ReturnDataBean {
		/**
		 * seid : 18000001010000000016
		 * nextStepKey : 02
		 * apduList : ["00A4040008A00000000300000000","80500000085D2EAC17C525B4EE00"]
		 */

		private String seid;
		private String nextStepKey;
		private List<String> apduList;

		public String getSeid() {
			return seid;
		}

		public void setSeid(String seid) {
			this.seid = seid;
		}

		public String getNextStepKey() {
			return nextStepKey;
		}

		public void setNextStepKey(String nextStepKey) {
			this.nextStepKey = nextStepKey;
		}

		public List<String> getApduList() {
			return apduList;
		}

		public void setApduList(List<String> apduList) {
			this.apduList = apduList;
		}

		@Override
		public String toString() {
			return "ReturnDataBean{" +
					"seid='" + seid + '\'' +
					", nextStepKey='" + nextStepKey + '\'' +
					", apduList=" + apduList +
					'}';
		}
	}

	@Override
	public String toString() {
		return "CommonResponse{" +
				"_ReturnCode='" + _ReturnCode + '\'' +
				", _ReturnMsg='" + _ReturnMsg + '\'' +
				", _ReturnData=" + _ReturnData +
				'}';
	}
}
