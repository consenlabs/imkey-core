package com.mk.imkeylibrary.device.model;

import com.mk.imkeylibrary.utils.TLVUtil;

import java.util.Arrays;

public class CertificateSCP11 extends TLVUtil {

	public static final byte OPTION_TAG_OFF = (byte) 0;
	public static final byte OPTION_LENGTH_OFF = (byte) 1;
	public static final byte OPTION_VALUE_OFF = (byte) 2;
	public static final byte OPTION_VALUE_LENGTH = (byte) 3;

	public CertificateSCP11(byte[] data) {
		super(data);
	}

	public CertificateSCP11(byte[] buffer, short offset, short length) throws Exception {
		super(buffer, offset, length);
	}

	public byte[] _93_csn;
	public byte[] _42_identifier;
	public byte[] _5F20_subjectID;
	public byte[] _95_keyUsage;
	public byte[] _5F25_EffectiveDate;
	public byte[] _5F24_ExpirationDate;
	public byte[] _53_DiscretionaryData;
	public byte[] _73_DiscretionaryData;
	public byte[] _BF20;
	public byte[] _7F49_PubKey;
	public byte[] _5F37_Signature;

	public byte[] get_93_csn() {
		return _93_csn;
	}

	public void set_93_csn(byte[] _93_csn) {
		this._93_csn = _93_csn;
	}

	public byte[] get_42_identifier() {
		return _42_identifier;
	}

	public void set_42_identifier(byte[] _42_identifier) {
		this._42_identifier = _42_identifier;
	}

	public byte[] get_5F20_subjectID() {
		return _5F20_subjectID;
	}

	public void set_5F20_subjectID(byte[] _5f20_subjectID) {
		_5F20_subjectID = _5f20_subjectID;
	}

	public byte[] get_95_keyUsage() {
		return _95_keyUsage;
	}

	public void set_95_keyUsage(byte[] _95_keyUsage) {
		this._95_keyUsage = _95_keyUsage;
	}

	public byte[] get_5F25_EffectiveDate() {
		return _5F25_EffectiveDate;
	}

	public void set_5F25_EffectiveDate(byte[] _5f25_EffectiveDate) {
		_5F25_EffectiveDate = _5f25_EffectiveDate;
	}

	public byte[] get_5F24_ExpirationDate() {
		return _5F24_ExpirationDate;
	}

	public void set_5F24_ExpirationDate(byte[] _5f24_ExpirationDate) {
		_5F24_ExpirationDate = _5f24_ExpirationDate;
	}

	public byte[] get_53_DiscretionaryData() {
		return _53_DiscretionaryData;
	}

	public void set_53_DiscretionaryData(byte[] _53_DiscretionaryData) {
		this._53_DiscretionaryData = _53_DiscretionaryData;
	}

	public byte[] get_73_DiscretionaryData() {
		return _73_DiscretionaryData;
	}

	public void set_73_DiscretionaryData(byte[] _73_DiscretionaryData) {
		this._73_DiscretionaryData = _73_DiscretionaryData;
	}

	public byte[] get_BF20() {
		return _BF20;
	}

	public void set_BF20(byte[] _BF20) {
		this._BF20 = _BF20;
	}

	public byte[] get_7F49_PubKey() {
		return _7F49_PubKey;
	}

	public void set_7F49_PubKey(byte[] _7f49_PubKey) {
		_7F49_PubKey = _7f49_PubKey;
	}

	public byte[] get_5F37_Signature() {
		return _5F37_Signature;
	}

	public void set_5F37_Signature(byte[] _5f37_Signature) {
		_5F37_Signature = _5f37_Signature;
	}

	/**
	 * Get the Sub TLV's option offset or value length, option can be @see
	 * OPTION_TAG_OFF.
	 * 
	 * @param tag
	 * @param option
	 * @return
	 * @throws Exception
	 */
	public short getSubTLVInfo(short tag, byte option) throws Exception {
		short offset = 2;
		short result = -1;
		byte[] buffer = this.data;
		short length = this.size();

		int len = TLVUtil.parseLength(buffer, offset, (short) 5);// Len(L)
		short valueLength7F21 = (short) (len & 0xffff);// Len(V)
		short lenLength7F21 = (short) (len >> 16);// L LEN
		offset += lenLength7F21;
		short endOffset = (short) (offset + valueLength7F21);

		while (offset < endOffset) {
			short tagLength = TLVUtil.parseTag(buffer, offset, length);
			short tagX = -1;
			if (tagLength == 1) {
				tagX = (short) ((byte) buffer[offset] & 0xFF);
			} else if (tagLength == 2) {
				tagX = TLVUtil.getShort(buffer, offset);
			} else {
				throw new Exception("certificate data error");
			}

			len = TLVUtil.parseLength(buffer, (short) (offset + tagLength), length);// Len(L)
			short valueLength = (short) (len & 0xffff);// Len(V)
			short lenLength = (short) (len >> 16);// L LEN

			if (tag == tagX) {
				switch (option) {
				case OPTION_TAG_OFF:
					result = offset;
					break;
				case OPTION_LENGTH_OFF:
					result = (short) (offset + tagLength);
					break;
				case OPTION_VALUE_OFF:
					result = (short) (offset + tagLength + lenLength);
					break;
				case OPTION_VALUE_LENGTH:
					result = valueLength;
					break;
				default:
					break;
				}
				break;// jump out while.
			}
			offset += tagLength;
			offset += lenLength;
			offset += valueLength;
		}

		return result;
	}

	/**
	 * Get the sub tag's value data.
	 * 
	 * @param tag
	 *            the sub tag in 7F21, See Table 6-12. GPCS 2.3 AmdF v1.1.0.23.
	 * @return
	 * @throws Exception
	 */
	public byte[] getSubTLVValue(short tag) throws Exception {
		byte[] result = null;
		byte[] buffer = this.data;
		short offset = 2;
		short length = this.size();

		int len = TLVUtil.parseLength(buffer, offset, length);// Len(L)
		short valueLength = (short) (len & 0xffff);// Len(V)
		short lenLength = (short) (len >> 16);// L LEN
		offset += lenLength;
		short endOffset = (short) (offset + valueLength);

		while (offset < endOffset) {
			short tagLength = TLVUtil.parseTag(buffer, offset, length);
			short tagX = -1;
			if (tagLength == 1) {
				tagX = (short) ((byte) buffer[offset]);
			} else if (tagLength == 2) {
				tagX = TLVUtil.getShort(buffer, offset);
			} else {
				throw new Exception("certificate data error");
			}

			len = TLVUtil.parseLength(buffer, (short) (offset + tagLength), length);// Len(L)
			valueLength = (short) (len & 0xffff);// Len(V)
			lenLength = (short) (len >> 16);// L LEN

			if (tag == tagX) {
				result = Arrays.copyOfRange(buffer, (offset + tagLength + lenLength),
						(offset + tagLength + lenLength + valueLength));
				break;
			}
			offset += tagLength;
			offset += lenLength;
			offset += valueLength;
		}

		return result;
	}

	/**
	 * getSignatureMessage
	 * @return
	 */
	public byte[] getSignatureMessage() {
		short offset = 0;
		short endOff = 0;
		try {
			short tagLen = TLVUtil.parseTag(this.data, offset, (short) 5);
			offset += tagLen;
			int dataLen = TLVUtil.parseLength(this.data, offset, (short) 5);
			
			short lenLen7F21 = (short) (dataLen >> 16);
			offset += lenLen7F21;

			endOff = getSubTLVInfo((short) 0x5F37, OPTION_TAG_OFF);
		} catch (Exception e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
		return Arrays.copyOfRange(this.data, offset, endOff);
	}

	/**
	 * getSignatureBytes
	 * 
	 * @return
	 */
	public byte[] getSignatureBytes() {
		byte[] sig = null;
		try {
			sig = getSubTLVValue((short) 0x5F37);
		} catch (Exception e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
		return sig;
	}

}