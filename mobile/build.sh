xcodebuild \
          -project benilla.xcodeproj \
          -scheme benilla \
          -sdk iphoneos \
          archive -archivePath ./benilla.xcarchive \
          CODE_SIGNING_REQUIRED=NO \
          AD_HOC_CODE_SIGNING_ALLOWED=YES \
          CODE_SIGNING_ALLOWED=NO \
          DEVELOPMENT_TEAM=XYZ0123456 \
          ORG_IDENTIFIER=org.benilla

ldid -benilla.entitlements benilla.xcarchive/Products/Applications/benilla.app/benilla

mkdir Payload
mkdir Payload/benilla.app
cp -R benilla.xcarchive/Products/Applications/benilla.app/ Payload/benilla.app/
zip -r ./app-release.ipa Payload