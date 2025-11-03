# Time

## About Time

Time is a software product that helps users to plan how they are going to spend their time working on activities and tracking the time spent. Time will help users to plan and improve their time management. Time is based around the Pomodoro Technique for time management. Using Time, users can perform pomodoros and Time will provide a counter for the user and record the information on each pomodoro performed. One pomodoro is 25 minutes in duration, followed by a 5 minute break. After 4 pomodoros, the user is encouraged to take a 30 minute break before beginning the next set of pomodoros. 

Time also will implement the Activity Inventory for the Pomodoro Technique and will track activities that the user is assigned to and needs to complete. Time implements the planning step for the Pomodoro technique. The first pomodoro of each day is a planning pomodoro and the user will choose activities from the Activity Inventory to work on each day. The user will prioritize the activities and then will begin performing pomodoros on each activity in priority order until the activity is completed.

## Getting Started

Before cloning the repository, please review the [software requirements](docs/software_requirements.md) and ensure that your development is properly configured. Once your development environment has the required software, you can clone the [Git repository](https://github.com/mfcollins3/time). In a terminal, navigate to the location in the file system where you want to host the repository and run:

    gh repo clone mfcollins3/time

This command will clone the [`mfcollins3/time`](https://github.com/mfcollins3/time) repository into the `time` subdirectory.

If you are using the development container for development, run Visual Studio Code and open the `time` directory as the workspace. You can do this in the terminal using the following command:

    code time

For local development, you will need to prepare the local repository for use and download the dependencies before you can build and run the Time software. In the terminal, run the following commands

```shell
cd time
./setup.sh
```

Microsoft Windows developers can use the PowerShell script instead:

```powershell
cd time
&./setup.ps1
```

Once the `setup` script has completed execution successfully, you can build and run the Time product locally in your development environment or machine.

## Using Time

Time is a terminal program with a command-line interface (CLI) interface and a Text UI (TUI) interface. To begin, open a terminal and run `time` to see what commands and options are available.

```plain
Available Commands:
  activity    Creates activities and manages the Activity Inventory
  completion  Generate the autocompletion script for the specified shell
  help        Help about any command
  mcp         Starts the Time MCP server for AI integration.
  pomodoro    Tracks and manages pomodoros for activities

Flags:
  -h, --help               help for time
      --log-level string   Set the logging level (debug, info, warning, error) (default "info")

Use "time [command] --help" for more information about a command.
```

### Managing the Activity Inventory

The __Activity Inventory__ is the list of all activities that have been assigned to you or that you have created to track work that you need to complete. You will complete these activities by focusing on them in iterations called __Pomodoros__. You can begin by creating a new activity using the `activity add` command:

    time activity add "Create my TPS Report"

This command will create the activity in the Activity Inventory and will output the identifier for the new activity. The identifier is output as a UUID value (`019a46d0-8a5f-7698-a5c5-d29b15cdec05`). You will need this value when you begin your first pomodoro.

### Performing a Pomodoro

To perform a Pomodoro, you can use the command:

    time pomodoro start

The Time program will start the Pomodoro timer and begin the countdown from 25 minutes. A progress bar will be shown in the terminal as the time counts down. The progress bar will turn to yellow after about 20 minutes (5 minutes remaining), and will turn red around 23:45 to indicate that the Pomodoro is almost complete. When the Pomodoro completes, an alarm should sound and you will also see a desktop notification.

![The Time program running and showing the Pomodoro timer counting down from 25 minutes with a progress bar moving from left to right](assets/pomodoro_timer.png)

> :exclamation: When running Time from a container, no alarm is played and there is no desktop notification. These features are not available when running from a container.

To link the pomodoro to an activity, you can use the `--activity` flag with the activity's identitifier:

    time pomodoro start --activity 019a46d0-8a5f-7698-a5c5-d29b15cdec05
