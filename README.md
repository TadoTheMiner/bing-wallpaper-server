This simple program runs a server that provides Bing images of the day at a single URL, so you can use it as wallpaper (I use it in the NightTab Firefox extension).
# Instalation
1. Run ```cargo install --git https://github.com/TadoTheMiner/bing-wallpaper-server/```
2. Add it to the end of your shell profile so it autostarts (make sure you run it in the background otherwise your profile will never stop executing)
3. You should have your image in http://127.0.0.1:8080/bing-daily-wallpaper
