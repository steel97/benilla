cargo ndk -t arm64-v8a -P 26 -o benilla/app/src/main/jniLibs build --release
cd benilla
./gradlew build