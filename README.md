To run using [cargo-apk](https://github.com/rust-mobile/cargo-apk):

# SDK 33

`cargo-apk apk run --target=aarch64-linux-android`

# SDK 30

`cargo-apk apk run --target=armv7-linux-androideabi`

# Two other targets that I'm not sure on which SDK they would run

`cargo-apk apk run --target=i686-linux-android`

`cargo-apk apk run --target=x86_64-linux-android`

I also tried to run these using [xbuild](https://github.com/rust-mobile/xbuild), in favor of which cargo-apk is depracted, but I couldn't get it to work.

Running:

`x devices`

To get <device_indentifier> and then:

`x run --device <device_identifier>`

For an emulator running SDK 33, produced a build for the target x86_64-linux-android, but it didn't run on the emulator, as one would expect, since the SDK 33 should be for aarch64-linux-android. The result was a linking error due to a missmatch of platforms.

Building for a physical device also failed. I tried to build it for my Samsung S8, running Android 9 (so SDK 28), but it also "Failed to collect all required libraries", which is caused by "Shared library `libaaudio.so` not found".

x doctor produces this result for me:

--------------------clang/llvm toolchain--------------------
clang 15.0.3 C:\Program Files\LLVM\bin\clang.exe
clang++ 15.0.3 C:\Program Files\LLVM\bin\clang++.exe
llvm-ar unknown C:\Program Files\LLVM\bin\llvm-ar.exe
llvm-lib unknown C:\Program Files\LLVM\bin\llvm-lib.exe
llvm-readobj 15.0.3 C:\Program Files\LLVM\bin\llvm-readobj.exe
lld 15.0.3 C:\Program Files\LLVM\bin\lld.exe
lld-link 15.0.3 C:\Program Files\LLVM\bin\lld-link.exe
lldb unknown C:\Program Files\LLVM\bin\lldb.exe
lldb-server unknown C:\Program Files\LLVM\bin\lldb-server.exe

----------------------------rust----------------------------
rustup 1.26.0 C:\Users\my_user\.cargo\bin\rustup.exe
cargo 1.72.0 C:\Users\my_user\.cargo\bin\cargo.exe

--------------------------android---------------------------
C:\Users\my_user\AppData\Local\Android\Sdk\platform-tools\adb.exe
C:\Users\my_user\AppData\Local\Programs\Eclipse Adoptium\jdk-17.0.6.10-hotspot\bin\javac.exe
java 17.0.6 C:\Users\my_user\AppData\Local\Programs\Eclipse Adoptium\jdk-17.0.6.10-hotspot\bin\java.exe
kotlin not found
gradle not found

----------------------------ios-----------------------------
idevice_id not found
ideviceinfo not found
ideviceinstaller not found
ideviceimagemounter not found
idevicedebug not found
idevicedebugserverproxy not found

---------------------------linux----------------------------
mksquashfs not found
