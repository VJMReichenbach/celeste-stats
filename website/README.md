# Plan

Create backend with rocket

~Create frontend with yew~ for now just make everything static

# Sites

- [ ] Index (with basic summary)
- [ ] Custom Map data
- [ ] Golden data
- [ ] Speedrun data

# Setup

Install sass:

```
yay -S --noconfirm dart-sass
```

Download bulma and move the sass files into the bulma folder (execute this in the `website` folder):

```bash
wget https://github.com/jgthms/bulma/releases/download/1.0.2/bulma-1.0.2.zip
unzip bulma-1.0.2.zip
rm bulma-1.0.2.zip
```

Compile the css file:

```bash
sass css/custom.scss:css/main.css
```
