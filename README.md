# evaporate

`evaporate` is a command line tool written in Rust that makes the process of extracting messages (iMessage and SMS) from an iOS (iPhone/iPad) backup. Don't be fooled by the multitude of tools online which claim to be free; most require a paid license or subscription after you've already downloaded it and viewed your messages. `evaporate` is *actually* free and open-source, so download and use as much as you want at no cost!

## Install

`evaporate` is not yet listed on major package repositories and so cannot yet be installed using `brew` or `apt`. As such, it must be compiled from source. This can be done by [installing Rust](https://www.rust-lang.org/tools/install) and then executing:

```
$ cargo install --git https://github.com/jeffreycshelton/evaporate
```

Assuming ~/.cargo (or the equivalent on Windows) is on PATH, you should be able to execute `evaporate`. If you ever want to uninstall `evaporate`, just execute this command:

```
$ cargo uninstall evaporate
```

## Usage

```
$ evaporate <path-to-backup> --name <name-of-contact>
```

***NOTE:*** The name provided must be exactly the first and last name stored in the phone's contacts, even if those fields contain extra characters like emojis, numbers, or symbols.

## Acknowledements

Huge thanks to Rich Infante for writing his guide [Reverse Engineering the iOS Backup](https://www.richinfante.com/2017/3/16/reverse-engineering-the-ios-backup). No code is directly copied from his guide, but many of the techniques shown in it are used in `evaporate`. The guide helped reduce the time required to write this tool significantly, and I greatly appreciate his effort in publishing it.

## License

`evaporate` is licensed under the [MIT License](LICENSE).
