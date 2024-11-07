# WayLyrics Compilation Guide for Ubuntu Users (2024.06.01)

## Contents 
- [Pre-viewing tips](#pre-viewing-tips)
- [Prepare environment](#Prepare-environment)
- [clone source code](#Clone-source-code)
- [Start compiling](#Start-compiling)
  - [Uses stable toolchain](#Uses-stable-toolchain)
  - [Use nightly Toolchain](#Use-nightly-toolchain)
- [Copy binary package to executable folder](#Copy-binary-package-to-executable-folder)
- [compile schemas](#compile-schemas)
- [Software icon Settings](#Software-icon-Settings)
- [Desktop Icon Settings](#Desktop-Icon-Settings)
- [Software theme Settings](#Software-theme-Settings)
- [compile end](#compile-end)
- [Waylyric some common tips](#waylyric-Some-commontips)
  - [Top lyrics](#Top-lyrics)

## Pre-viewing tips
All commands are run by common users. Understand the function of each command before running it. If you encounter permission issues, use sudo as appropriate.

Before compiling, please make sure that your network communication is good, otherwise it is likely to get stuck at the beginning of compiling.

This document has Waylyrics installed globally by default, and advanced users can put some of the files into ~/.local/share.

Alternatively, you can install a compiled binary package from [Spark Store](https://spark-app.store/).
```shell
sudo aptss install waylyrics
```

## Prepare the environment
```shell
sudo apt-get install git nano build-essential libssl-dev libgtk-4-dev libdbus-1-dev libmimalloc-dev gettext cargo
```
📣Tips: The cargo package here pulls up the rustc package, note that the rustc package version needs to be >= 1.73. Advanced users can build their own rust environments instead of installing cargo from the package manager.

## Clone source code
We need to clone the Waylyrics source code on Github for compilation. Make sure the Internet connection is good, and in rare cases use the magic 🪄.
```shell
mkdir gittemp
cd gittemp
# Here the mkdir command creates a folder named gittemp and uses the cd command to switch the working directory of the command line so that the cloned project does not contaminate the home directory.
git clone https://github.com/waylyrics/waylyrics.git
# clone the Waylyrics project source code using git command.
cd waylyrics
# Use the cd command to switch the command line working directory to the project folder.
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
# Set the Waylyrics theme directory variable, which will be used later
```
💡Tips: Advanced users look over meow! Noobs can skip it.
```shell
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
```
This environment variable is used to specify the directory where Waylyrics loads the theme at compile time. You can change it yourself, but without specifying this variable, Waylyrics loads the theme from ${XDG_DATA_HOME}/_themes/ by default.




## Start compiling
There is a stable standard toolchain at compile time, or the nightly daily update toolchain can be selected (nightly is not recommended by Noobs).

⚠️Waring: For the third time, please ensure that your network is free, the download of the tool chain ahead requires a lot of traffic.

For users who really don't have magic, we offer the cargo swap source to download the toolchain!

```shell
nano ~/.cargo/config.toml
# Edit the cargo profile in the user directory using the nano text editor
```

Copy the following text into the text, where the text is [rsproxy.cn source](https://rsproxy.cn/#getStarted)

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```
Then press Ctrl+X to enter exit mode, type "Y" and press Enter to save.


### Use the stable toolchain
```shell
cargo build --release --locked --target-dir target
# Start compiling with the default Settings and output the compiled binary package to the./target/release/ folder.
```

### Use the nightly tool chain
```shell
cargo +nightly build --release --locked --target-dir target
# Start the compilation using the daily update toolchain and output the compiled binary package to the./target/release/ folder.
```

## Copy the binary package to the executable folder
On Linux, the /bin folder contains a lot of executables such as apt, grep, etc. We copied the waylyrics binary to /usr/bin and then created a symbolic link to /bin from our compiled software. This allows us to launch the waylyrics program directly with the Waylyrics command.
```shell
sudo cp ./target/release/waylyrics /usr/bin
# Copy the waylyrics binary to /bin
sudo chmod 755 /usr/bin/waylyrics
# Set the /usr/bin/waylyrics permission to 755
sudo ln -s /usr/bin/waylyrics /bin/waylyrics
# Create a file link that gives system access to /bin/waylyrics direct access to /usr/bin/waylyrics
```
💡Tips: Why not just put the file in /bin and access it directly? In fact, this is OK, but in order to standardize the location of each file, we should consciously put binaries in /usr/bin, rather than directly into /bin. There is doubt can go to the about[/bin 和 /usr/bin](https://unix.stackexchange.com/questions/5915/difference-between-bin-and-usr-bin) to discuss!

## Compile schemas
Waylyric needs to allow applications to read and write user Settings through the GSettings API (the calling interface for the GNOME desktop environment Settings storage and retrieval system, because Waylyric is GTK software, so we need to call this interface). Let's start compiling schemas.
```shell
Cp. The metainfo/IO. Making. Waylyrics. Waylyrics. Gschema. XML/usr/share/glib - 2.0 / schemas
Copy the schemas file to /usr/share/glib-2.0/schemas/
glib-compile-schemas /usr/share/glib-2.0/schemas/
# Compile the schemas file
```

## Software Chinese
📣Tips: This option is optional and does not affect the normal use of the software

What? ! When I launch the software, it's all in English? ! Don't worry, we just need to compile the language file to Chinese Waylyrics!
```bash
cd locales/zh_CN/LC_MESSAGES/
msgfmt waylyrics.po
# Compile.po language files into.mo files
sudo cp ./messages.mo /usr/share/locale/zh_CN/LC_MESSAGES/waylyrics.mo
```

## Software icon Settings
📣Tips: This item is important and may affect the software use experience

We need to set up a nice Logo for the software! Our adorable Waylyrics, of course! Let's bring the cute little Fu Fu into the system!
```shell
cp -r ./res/icons/* /usr/share/icons/
Copy all files under./res/icons/ to the icon folder specified by the system
```

## Desktop icon Settings
📣Tips: This item is important and may affect the software use experience

We needed a startup icon on the desktop to manipulate Waylyrics, and the source code provided us with a ready-made desktop file to use, so we just copied that file into the folder where the desktop icon was stored.
```shell
# Make sure that the working directory of the command line is under the project directory and not under subdirectories such as locales, metainfo, etc
sudo cp ./metainfo/io.github.waylyrics.Waylyrics.desktop /usr/share/applications/
# Copy the Waylyrics desktop file to /usr/share/applications/
sudo chmod 644 /usr/share/applications/io.github.waylyrics.Waylyrics.desktop
# Set permissions for desktop ICONS to 644
```
## Software theme setting
📣Tips: This option is optional and does not affect the normal use of the software

After installing the software, you also need a beautiful theme, just as Waylyric supports theme configuration, we need to copy the default theme to the standard directory.
```shell
mkdir -p /usr/share/waylyrics/themes/
# to create/usr/share/waylyrics/themes/it a bunch of folders
cp -r ./themes/* /usr/share/waylyrics/themes/
# copy. All files under/themes/to/usr/share/waylyrics/themes /
sudo chmod 755 -R /usr/share/waylyrics/themes/
# Set folder permissions to 755
```

## Finish compiling
At this point, you should have completed the entire installation process, isn't it interesting? But everything has passed, the software compilation process has been completed!

## Waylyric Some common tips to use
### Top lyrics
KDE users: Keep Waylyrics in the window, right-click on the title bar, select more, select Top, and then close the window to top lyrics. If you want to achieve lasting set-top, window, please refer to the [discuss] (https://github.com/waylyrics/waylyrics/discussions/181).

GNOME users: Leave the Waylyrics window as it is, right click on the title bar and click to the top. Because Waylyrics is a GTK application, Waylyrics is more thematic integrated on the GNOME desktop.
