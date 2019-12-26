pushd ../proto/src
rm -rf ../../ios/imKeyConnector/Classes/Proto
mkdir ../../ios/imKeyConnector/Classes/Proto

protoc --swift_out=../../ios/imKeyConnector/Classes/Proto api.proto
protoc --swift_out=../../ios/imKeyConnector/Classes/Proto device.proto
protoc --swift_out=../../ios/imKeyConnector/Classes/Proto eth.proto
popd