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
- **Audio**: Audio files.
- **Text & Code**: Text and Code files.
- **Work**: MS office type files.
- **Audio**: Zipped folder (zip, rar, tar).
- **Other**: Any file that couldn't be identified as any of the above categories.

<img src="./assets/demo.png" style="width: 150px"/>

# Installation

1. Clone this repository somewhere that you will **not be sorting** (I recommend putting it somewhere in your root directory).
<br>
2. Add the path to the **sort-it.exe** and **unsort-it.exe** to your environment variables. 
    - The path to **sort-it.exe** is **[WHERE-YOU-PUT-IT]/files/target/release/** 
    - The path to **unsort-it.exe** is **[WHERE-YOU-PUT-IT]/unsort/target/release/** 
    - **Do not include the actual .exe in that path, only the release which is where it is**
    <br>
    **This is what mine looks like in my Environment Variables > System Variables > Path**:

    <img src="./assets/mypaths.png" style="width: 750px"/>

<br>

3. Open a terminal and navigate to the directory you'd like to sort (or just right click in the directory, and select 'Open In Terminal', which is better in my opinion), and type **'sort-it'** or **'unsort-it'** to sort/unsort your directories!

# Notes
- **If you have any file/folder that you DON'T want sorted into it's indentified sort directory**, you can just put it in "**ٴOTHER**".
<br>
- **unsort-it** is specifically meant to unsort a directory that has already been sorted by **sort-it**, it specifically looks to unsort the folders that **sort-it** creates.
<br>
- This currently will not sort your directories automatically, a program like that would use an unecessary amount of your CPU. **However, I currently adding a feature which sorts all of your previously sorted folders on startup.**

# Contact Me

Feel free to contact me with any questions or concerns at: oneilb123@gmail.com