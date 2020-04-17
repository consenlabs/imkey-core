pushd ../proto/src
#rm -rf ../../ios/imKeyConnector/Classes/Proto
#mkdir ../../ios/imKeyConnector/Classes/Proto
#
#protoc --swift_opt=Visibility=Public --swift_out=../../ios/imKeyConnector/Classes/Proto api.proto
#protoc --swift_opt=Visibility=Public --swift_out=../../ios/imKeyConnector/Classes/Proto device.proto
#protoc --swift_opt=Visibility=Public --swift_out=../../ios/imKeyConnector/Classes/Proto btc.proto
#protoc --swift_opt=Visibility=Public --swift_out=../../ios/imKeyConnector/Classes/Proto eth.proto
#protoc --swift_opt=Visibility=Public --swift_out=../../ios/imKeyConnector/Classes/Proto eos.proto
#protoc --swift_opt=Visibility=Public --swift_out=../../ios/imKeyConnector/Classes/Proto cosmos.proto

rm -rf ../../examples/ios/iosExample/Proto
mkdir ../../examples/ios/iosExample/Proto

protoc --swift_opt=Visibility=Public --swift_out=../../examples/ios/iosExample/Proto api.proto
protoc --swift_opt=Visibility=Public --swift_out=../../examples/ios/iosExample/Proto device.proto
protoc --swift_opt=Visibility=Public --swift_out=../../examples/ios/iosExample/Proto btc.proto
protoc --swift_opt=Visibility=Public --swift_out=../../examples/ios/iosExample/Proto eth.proto
protoc --swift_opt=Visibility=Public --swift_out=../../examples/ios/iosExample/Proto eos.proto
protoc --swift_opt=Visibility=Public --swift_out=../../examples/ios/iosExample/Proto cosmos.proto
popd