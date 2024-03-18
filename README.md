# Sort It

**This program sorts every file and folder inside of a directory with one command!**

# How are your folders sorted?

#### Your files are sorted into 10 different folders:

- **_FOLDERS**: Folder Type files.
- **Apps**: Application Type files (Installers, executables, etc.).
- **Audio**: Audio files.
- **Video**: Video files.
- **Images**: Image files.
- **PDF**: PDF files.
- **Text & Code**: Text and Code files.
- **Work**: MS office type files.
- **Audio**: Zipped folder (zip, rar, tar).
- **Other**: Any file that couldn't be identified as any of the above categories.

#### Please note that any of the above folders will only be created if your directory contains at least one file of their respective type. For instance, if there are no audio files in the directory you are sorting, the 'Audio' folder will not be created.

![Demo](./assets/demo.png)


**If a file that is to be moved has a duplicate in its target folder, the date and time will be appended to the duplicate so that nothing is overwritten. This doesn't work with duplicate folder names, where the original folder has files inside of it. In this case, the folder simply won't be moved. You DON'T have to worry about anything being overwritten.**

# Installation (Windows)

1. Clone this repository somewhere that you will **not be sorting** (I recommend putting it somewhere in your root directory).
2. Add the path to 'sort-it.exe' and 'unsort-it.exe' to your environment variables.
   - The path to 'sort-it.exe' and 'unsort-it.exe is '[PATH-TO-REPO]/Sort-It/'
   - Do not include the actual .exe in that path, only the path up to and including the repo.
   - This is what mine looks like in my Environment Variables > System Variables > Path:
   
<img src="assets/mypaths.png" alt="drawing" width="400"/>

3. Type 'cmd' in the address bar of the directory you'd like to sort (or just cd into it through powershell), and type **'sort-it'** or **'unsort-it'** to sort/unsort your directories!

![Demo](./assets/demo2.png)

# Installation (Linux & MacOS)

1. As of right now Linux & MacOS users must have **'wine'** installed as **'sort-it'** and **'unsort-it'** files are Windows .exe files, and wine allows Unix systems to run exe files in their environemnt.
   - You can find instructions for installing wine [here](https://wiki.winehq.org/Download).
2. Once wine is installed, simply clone the repository somewhere that you **do not** plan to sort (I recommend your home directory, which makes the command simpler).
3. Make sure that you give **sort-it** and **unsort-it** execution privileges if necessary.
4. Then you can run ```wine [PATH-TO-REPO]/Sort-It/sort-it.exe```
![Demo](./assets/demo3.png)


# Contact Me

Please feel free to contact me with any questions or concerns at: oneilb123@gmail.com
