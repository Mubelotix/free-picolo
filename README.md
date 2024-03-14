# Free-picolo

This is an alternative implementation of the Picolo drinking game.
All the content from the original game is available for free in this app.

## Documentation

### How to update data.json

- Download the APK of the original app (using something like [this site](https://www.apkmonk.com/app/com.picolo.android/))
- Decompile the APK using JADX `jadx -d picolo com.picolo.android.apk`
- Install Realm Studio and open the file `picolo/resources/assets/default-7.realm`
- Press the File>Save Data>JSON
- Move the obtained file to the root of this project and rename it to `data.json`
- (Optional) From the json, remove the useless array containing numbers from 2 to 65000 as strings
