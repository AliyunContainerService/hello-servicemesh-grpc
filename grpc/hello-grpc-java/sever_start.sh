sh build.sh
export JAVA_HOME=$JAVA_16_HOME
mvn exec:java -Dexec.mainClass="org.feuyeux.grpc.server.ProtoServer"