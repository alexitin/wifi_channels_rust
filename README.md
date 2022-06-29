# Wifi noise analyzer
Helper for choice wifi chanel.

This is a training project.
The purpose of this project is to learn practical skills in programming.
The task that this project solves is the determination of the least filled Wi-Fi channel.
To solve this problem, the following actions are performed:
1. Searches for Wi-Fi devices and checks if they support monitor mode.
2. Attempt to establish a connection with the capture of radio data. It is sometimes possible to capture radio data in promiscouos or normal modes.
3. Scanning wifi channels to determine ssid and their signal strength.
4. Exclude from home network scan data.
5. Display for each channel the number of WiFi access points and the maximum signal level.