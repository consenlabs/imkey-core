package com.mk.imkeylibrary.utils;

public class TLVUtil {


	public static short calcLengthSize(int vLen) {
		short lLen = 0;
		if (vLen < 128) {
			lLen = 1;
		} else if (vLen < 256) {
			lLen = 2;
		} else if (vLen < 65536) {
			lLen = 3;
		} else {
			lLen = 4;
		}
		return lLen;
	}

	public static short calcLengthValue(byte[] dist, short distOffset, int valueSize) {
		if (valueSize < 128) {
			dist[distOffset++] = (byte) valueSize;
		} else if (valueSize < 256) {
			dist[distOffset++] = (byte) 0x81;
			dist[distOffset++] = (byte) valueSize;
		} else if (valueSize < 65536) {
			dist[distOffset++] = (byte) 0x82;
			dist[distOffset++] = (byte) ((valueSize >> 8) &0xFF);
			dist[distOffset++] = (byte) (valueSize&0xFF);
		} else {
			dist[distOffset++] = (byte) 0x83;
			dist[distOffset++] = (byte) ((valueSize >> 16) &0xFF);
			dist[distOffset++] = (byte) ((valueSize >> 8) &0xFF);
			dist[distOffset++] = (byte) (valueSize&0xFF);
		}
		return distOffset;
	}
	protected static short verifyFormat(byte[] buffer, short offset, short length) {
		short ret = 0;
		short tagLen = parseTag(buffer, offset, length);
		if (tagLen < 0) {
			return tagLen;
		}
		ret += tagLen;
		offset += tagLen;
		length -= tagLen;
		int len = parseLength(buffer, offset, length);

		ret += (len >> 16);
//		offset += (len >> 16);
		length -= (len >> 16);
		if (length < (len & 0xffff)) {
			return (short) -1;
		}
		ret += (short) (len & 0xffff);
		return ret;
	}

	protected static short parseTag(byte[] buffer, short offset, short length) {
		short tagOff = offset;
		if (length < 1) {
			return (short) -1;
		}
		byte tag = buffer[tagOff++];
		if ((tag & 0x1f) == 0x1f) {
			while ((buffer[tagOff] & 0x80) != 0) {
				tagOff++;
				if (tagOff - offset > length) {
					return (short) -1;
				}
			}
			tagOff++;
			if (tagOff - offset > length) {
				return (short) -1;
			}
		}
		return (short) (tagOff - offset);
	}

	protected static int parseLength(byte[] buffer, short offset, short length) {
		if (length < 1) {
			return (short) -1;
		}
		byte len = buffer[offset++];
		if (len >= 0) // 0~7F
		{
			return 0x010000 | (len & 0xff);
		} else if (len == (byte) 0x81) {
			if (length < 2) {
				return (short) -1;
			}
			return 0x020000 | (buffer[offset] & 0xff);
		} else if (len == (byte) 0x82) {
			if (length < 3) {
				return (short) -1;
			}
			short val = getShort(buffer, offset);
			if (val < 0) {
				return (short) -1;
			}
			return 0x030000 | val;
		} else {
			return (short) (-1);
		}
	}

	protected byte[] data;
	short dataSize;

	public TLVUtil(byte[] buffer, short offset, short length) throws Exception {
		short size = verifyFormat(buffer, offset, length);
		if (size < 0) {
			throw new Exception("tlv size error.");
		}
		try {
			byte[] tempData = new byte[size];
			System.arraycopy(buffer, offset, tempData, (short) 0, size);
			this.data = tempData;
			dataSize = size;
		} catch (Exception e) {

		}
	}
	public TLVUtil(byte[] tag, byte[] data) {

		short tagLen = parseTag(tag, (short)0, (short) tag.length);
		short lengthSize = TLVUtil.calcLengthSize(data.length);

		byte[] ret = new byte[tagLen + lengthSize + data.length];

		System.arraycopy(tag, 0, ret, 0, tagLen);
		short offset = TLVUtil.calcLengthValue(ret, tagLen, data.length);
		System.arraycopy(data, 0, ret, offset, data.length);

		//short size = verifyFormat(data, (short) 0, (short) data.length);
		this.data = ret;
		this.dataSize = (short) ret.length;
	}

	public TLVUtil(byte[] data) {
		short size = verifyFormat(data, (short) 0, (short) data.length);
		this.data = data;
		this.dataSize = size;
	}

	public TLVUtil(byte[] data, short size) {
		this.data = data;
		this.dataSize = size;
	}

	public static short getShort(byte[] bytes, short offset) {
		return (short) ((0xff00 & (bytes[offset] << 8))|(0xff & bytes[offset + 1]));
	}

	public boolean tagEquals(short tag) {
		short tagLen = parseTag(data, (short) 0, (short) this.dataSize);
		if (tagLen == 1) {
			return tag == (short) (data[(short) 0] & 0xff);
		} else if (tagLen == 2) {
			return tag == getShort(data, (short) 0);
		} else {
			return false;
		}
	}

	public short size() {
		return (short) dataSize;
	}

	public short toBytes(byte[] buffer, short offset) {
		short size = (short) dataSize;
		System.arraycopy(data, (short) 0, buffer, offset, size);
		return (short) (offset + size);
	}


	public short getValue(byte[] buffer, short offset) {
		short tagLen = parseTag(data, (short) 0, (short) dataSize);
		int len = parseLength(data, tagLen, (short) (dataSize - tagLen));
		short valueLen = (short) (len & 0xffff);
		short lenLen = (short) (len >> 16);

		System.arraycopy(data, (short) (tagLen + lenLen), buffer, offset, valueLen);
		return valueLen;
	}

}