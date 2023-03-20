# A Link-shortener with a React frontend and Rust backend

The first non trivial project for both Rust and React I have worked on.
It was a great learning experience but the project is on hold for now.

There are some issues which I didnt get around to fix yet:
- This program only works for my url at the moment because the program is compiled with my url hardcoded.
- It is possible to build loops that point to my service that could seriously slow down the application if used maliciously
- I am not utilizing Docker Volumes at the moment. Therefor data is a little unsafe 
