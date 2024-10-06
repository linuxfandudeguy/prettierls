
# PrettierLS

PrettierLS is a colorful tree command that enhances the visualization of file structures in your terminal. It supports various file types and provides a customizable output with ANSI colors.

## Features

- Color-coded output based on file types and extensions.
- Supports common programming languages and file types.
- Easy installation via a simple shell command.

## Installation

You can easily install PrettierLS using the following command:

```bash
curl https://linuxfandudeguy.github.io/prettierls/install.sh | bash
```
You can also uninstall PrettierLS with this command:

```bash
sudo rm /usr/local/bin/prettierls
```

### How It Works

1. The installation script will check if Rust is installed on your system.
2. If Rust is not installed, it will prompt you to install it.
3. After confirming Rust installation, it will download the PrettierLS binary and place it in your system PATH for easy access.
4.If you want to do a certain directory you will have to manually navigate to that directory and then type `prettierls`
## Usage

Once installed, you can use PrettierLS in the terminal by simply typing:

```bash
prettierls
```

This will display a color-coded tree structure of your current directory. You can customize the output by exploring additional options.

## Contribution

If you want to contribute to PrettierLS, feel free to fork the repository and submit a pull request. 

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

