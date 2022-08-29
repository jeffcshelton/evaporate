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

First, your backup _must_ be **unencrypted**. In the future, `evaporate` may support encrypted backups if the password is known, but not currently. Then, find the file path to your backup and run this command in your terminal:

```
$ evaporate <path-to-backup> -o <path-to-extraction>
```

The backup will then be extracted to the specified output path in a human-readable format. Currently, it extracts messages, photos, and contacts from the backup into the following format:

```
<output-path>
  | contacts.txt
  | messages
      | <contact-1>.txt
      | <contact-2>.txt
      .
      .
  | photos
      | IMG_0001.JPG
      | IMG_0002.JPG
      .
      .
```

If you want to see more features added, please consider contributing! If there is enough demand, I will make a contribution guide document. Otherwise, please still create issues and pull requests to help out with the project.

## Acknowledements

Huge thanks to Rich Infante for writing his guide [Reverse Engineering the iOS Backup](https://www.richinfante.com/2017/3/16/reverse-engineering-the-ios-backup). No code is directly copied from his guide, but many of the techniques shown in it are used in `evaporate`. The guide helped reduce the time required to write this tool significantly, and I greatly appreciate his effort in publishing it.

## License

`evaporate` is licensed under the [MIT License](LICENSE).
