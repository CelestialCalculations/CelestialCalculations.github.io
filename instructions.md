Software for [_Celestial Calculations_](https://mitpress.mit.edu/books/celestial-calculations) is provided in three flavors: Java, Python 3, and Visual Basic. You may use whichever programming language you prefer, or even all three. The programs are identical regardless of which language(s) you elect to use. Installation is simple and straightforward.

# Install the Executables and Data Files
1. Download the "executables and data" zip file from the repository to your local machine.
1. Unzip the downloaded zip file to any convenient disk and directory on your local machine (e.g., C:\CELCALC).
1. The unzipped file will create a subdirectory "Ast Progs and Data" on your local machine (e.g., C:\CELCALC\Ast Progs and Data). A README.txt file is also created at the same level as the "Ast Progs and Data" directory. The README file contains supplementary information about the programs and data.
1. The "Ast Progs and Data" directory contains the executable programs and data. Do **NOT** change the name of this directory or change the directory structure underneath it.
1. The executables are underneath "Ast Progs and Data" in "bin-Java" (executable Java jar files), "bin-Python" (Python scripts), and "bin-VBasic" (Visual Basic executables compiled for Windows platforms. Non-Windows users (e.g., iOS) must recompile the Visual Basic source code for their platform. The Java and Python programs are not platform-specific.)

You may safely delete any of the "bin-" subdirectories for the languages you do not want to use. For example, if you do not want the Visual Basic executables, delete the "bin-VBasic" subdirectory (e.g., delete C:\CELCALC\Ast Progs and Data\bin-VBasic).

# Install the Source Code (Optional)
1. Download the "source code" zip file from the repository to your local machine.
1. Unzip the downloaded zip file to any convenient disk and directory on your local machine (e.g., C:\CELCALC). It is strongly recommended, but not required, that you install the source code in the same subdirectory as the executables and data files.
1. The unzipped file will create 3 top level directories on whatever disk and underneath whatever directory you elected to use. The directories are: "Java", "Python", and "Visual Basic".
1. The directories "Java", "Python", and "Visual Basic" contain the source code that can be imported into your preferred development environment.

Assuming that you elected to install the executables and data in the directory C:\CELCALC and that you elected to also install the source code in the same directory, your directory structure should look like this:
```
C:\CELCALC
   |--Ast Progs and Data
          |--Ast Data Files
          |--bin-Java
          |--bin-Python
          |--bin-VBasic
   |--Java
   |--Python
   |--Visual Basic
   |--README.txt
 ```

Unless you wish to modify the source code, you do not have to copy any source code directories to your system. The downloaded zip file contains all three versions (Java, Python 3, Visual Basic) of the software. You may leave all three versions installed on your machine, or delete the ones that you do not want from underneath both "Ast Progs and Data" (for the executables) and the relevant source code directory. For example, if you do not want the Visual Basic version of the software, delete the subdirectory "Ast Progs and Data\bin-VBasic" and the source code directory "Visual Basic".

# Run the Programs
Executables in the "bin-" subdirectories are named **RunChapX** where _**X**_ is the chapter to which the program applies. Thus, **RunChap1.jar** is the Java jar file for the book's Chapter 1, **RunChap2.py** is the Python script for Chapter 2, **RunChap3.exe** is the Visual Basic executable (for Windows!) for Chapter 3, etc. Run the programs as you would any other program on your machine (double click on the executable, execute from a command line with a command such as _java -jar RunChap1.jar_ or _python RunChap8.py_, etc.).

# Remove the Programs, Data, and Source Code
Installing and running the programs does not make any changes to the Windows registry (for Windows-based platforms), nor are any special steps required to install the executables, data, and source code. The programs and data can be removed from your system by merely deleting the "Ast Progs and Data" directory, source code directories, and README.txt file from wherever you unzipped them.
