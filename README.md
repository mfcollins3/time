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
