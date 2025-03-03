# What is it?
A basic API that compiles for Windows and Linux with the goal of allowing remote shutdown and restart by way of something like Home Assistant.

# How to Use?
Currently the program needs to be compiled and setup in the start folder on Windows, or setup as a service in Linux.
Then by calling:

 - localhost:5002/test
 - localhost:5002/api/v1/restart
 - localhost:5002/api/v1/shutdown
 - localhost:5002/api/v1/quit

You can test the connect, restart the machine, shutdown, or end the api, respectively. If this is running on a different machine then you would replace 'localhost' with the IP of that machine.

# Security? 
Security is taken pretty lightly as this is meant to be ran locally and not across (or exposed to) the open internet. If you want to use a key you can pass the key as a parameter:

/path/to/system_api key123

This will make the commands only work with the following pattern:

 - localhost:5002/test
 - localhost:5002/api/v1/key123/restart
 - localhost:5002/api/v1/key123/shutdown
 - localhost:5002/api/v1/key123/quit

# To-Do:
 - Package a compiled executable for both Windows and Linux
 - Include a template or script for setting up as a Linux service. Windows can just place the exe in "shell:startup" in File Explorer  