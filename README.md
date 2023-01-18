<h1 align="center"><i><u>Disrust</u></i></h1>

---

![Peek 2023-01-17 17-34](https://user-images.githubusercontent.com/96934612/213059486-3947adba-2700-4f14-bcfc-adf5ed5e4a83.gif)

# What is this ?
Disrust is a TUI discord client written entirely in glorious Rust. It works*. 

The app is not fully feature complete. It's missing a lot of discord features, but the basics like navigating between servers, viewing channels, sending messages, etc are there. I might eventually come back to clean up code and make it fancier. 

It's not fully complete because I really badly wanted to move on to my "infinite storage glitch" project idea and because every 10 minutes spent on doing anything UI related is shaving off at least a year of my lifespan each time.

I made it mostly to get better at Rust and because it's fun to work on a larger project for once. 

---

# Now, you might be asing yourself:

<details>
<summary><b>But is this legal ?</b></summary>
<b>No.</b>

The use of this app is very much not kosher according to discord's TOS. I do not recommend using it seriously.
</details>

# Controls
- Paste your user token in when the program prompts it. If you don't know what that is, look up a video on youtube or smthn
- Use arrows to navigate.
- Press ```e``` to enter editing mode and ```esc``` to leave it
- Press ```enter``` to open a server and view channels or send a message if you are in editing mode
- Press ```esc``` to leave a server if not in editing mode
- Press ```q``` to quit the app

# Credits and final comments

Thanks to <b>Traumatism</b> (https://github.com/Traumatism) and their project <b>ToastCord</b> (https://github.com/Traumatism/ToastCord) for guidance. This would have been a lot more confusing to code without them.

Thanks to <b>6cord</b> (https://github.com/diamondburned/6cord) for the initial inspiration

I appreciate any and all roasting of the code so I can improve. 

Feel free to modify this code however you like (As long as the license remains GPL 3.0) or use for reference on how to interact with discord's API and specifically gateway, it was a pain in the ass to setup. I wish I had a reference like this in Rust when I started. 

The API in general is pretty neat btw. I recommend messing with it or maybe starting a project with it.
