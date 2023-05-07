# Task Fusion

Task Fusion is a system monitor created for Linux and mainly targets Ubuntu Machines. It has both a command-line and GUI edition.

## Table of Contents

- [Instructions](#instructions)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Creators](#creators)
- [Special Thanks](#special-thanks)

## Getting Started

1. Open a terminal and navigate to the `taskfusion_gui` directory.
2. Run the following command to make the `taskfusion_gui` executable:

    ```bash
    chmod +x taskfusion_gui
    ```

   This command will set the executable permission for the `taskfusion_gui` file, allowing it to be executed.

3. Once the `taskfusion_gui` file is made executable, you can choose to run it as a standalone entering the following command in the terminal:

    ```bash
    ./taskfusion_gui
    ```

4. Run the following command to make the `taskfusion` executable:

    ```bash
    chmod +x taskfusion
    ```

   This command will set the executable permission for the `taskfusion` file, allowing it to be executed.


## Usage

To start `taskfusion_gui` as a standalone, run the following command: 

```bash
./taskfusion_gui
```
To run the entire project, use the following command:

```bash
./taskfusion -ls 
```
This takes you to the home screen that shows all the processes, you can also access specific pages of the app from the app or from terminal using flags:-


1- you can filter by PID 
``` bash
./taskfusion -p <pid>
```

2- Similarly, by state
```bash
./taskfusion -s <state>
```
3- by CMD:
```bash
./taskfusion -c <cmd>
```

4- by GID:
```bash
./taskfusion -g <gid>
```

5- by ppid 
```bash
./taskfusion -pp <gid>
```

6- To kill a process: 
```bash
./taskfusion -k <pid>
```

7- To kill a process and its children:-
```bash
./taskfusion -kp <pid>
```

8- To pause a process:-
```bash
./taskfusion -pa <pid>
```

9- to resume:-
```bash
./taskfusion -r <pid>
```

10- to change priority:-
``` bash 
./taskfusion -cpr <pid priority>
```

11- to take a time stamped snapshot:- 
``` bash 
./taskfusion -t
```

12- to kill processes exceeding certain CPU usage threshold:-
``` bash 
./taskfusion -kt <threshold>
```
13- to view the process tree
```bash
./taskfusion -tree
```
## Creators

The creators of this project are Omar Elwaliely, Salma Aly, Muhammad El-Mahdi, and Salma Kaffafy of The American University in Cairo.

## Special Thanks

Special thanks to Dr. Amr El Kadi for supervising the project.
