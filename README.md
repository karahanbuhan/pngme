# pngme
Command line program that lets you hide secret messages in PNG files.

## About
**pngme** is a [Command Line Interface](https://en.wikipedia.org/wiki/Command-line_interface) to modify [chunks of PNG files](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html).

![pngme](https://user-images.githubusercontent.com/83908403/201533876-e714383a-b398-4297-88f4-b11a41c7ff97.png)

## Usage

### Adding a message
To insert a secret message to a PNG file, run: `pngme encode file.png chNk "Your message here"`

*chNk* is the type of chunk that your message will be written on. You must use 4 ASCII Alphabetic (a-Z) characters as the type and 3rd byte (character) starting from left **should** be uppercase. So `ruSt`, `heLo`, `pnGm` are good examples. For more details about chunk types, please visit http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

### Reading a message
To read a secret message in a PNG file, run: `pngme decode file.png chNk`

This will print the message in the `chNk` if there is one.

### Finding messages
To find messages, you can use `pngme print file.png`

This will list all the chunks so be aware. Data of the image is also in chunks and you will not be able to read them because they are not UTF-8 texts.

### Removing messages
To remove message (chunk), you can use `pngme remove file.png chNk`

You can accidentally delete your image if you were to remove the image data chunks. Only remove the messages you created! If there is duplication of chunk types, this command will remove the first one so beware.

### I don't understand, please help me!
Run the executable for help as: `pngme --help`

## License
[The MIT License](https://opensource.org/licenses/MIT)
