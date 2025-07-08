# Minecraft Server Utilites

This is a basic command line tool that aims to help trivialise the setup of a basic vanilla Minecraft server.
It supports downloading versions of a server from the internet, saving them in the home directory or another specified folder and changing it's properties.
You can also download any type of server if you add the download manifest to the settings. Currently, it has the [vanilla Minecraft manifest](https://launchermeta.mojang.com/mc/game/version_manifest.json) built in, but I have not added the manifests for any modded servers as I could not find them. Any custom manifests will need to follow the formatting of the regular Minecraft json files, and I cannot guarantee that this will work

The TUI also supports theming through the [cursive library's theming support](https://docs.rs/cursive/latest/cursive/theme/index.html). Simply creates a file called ```theme.toml``` in the executable directory and specify your theme parameters there.

This project was made for the Hackclub Summer of Making event.