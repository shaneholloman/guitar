<div align="center">
  <pre>
    </br>
                              :GG~        .?Y.                                
    ....        ..      ..   .....      . ^BG: ..       .....                 
 .7555YY7JP^   ~PJ     ~PJ  ?YY5PP~    7YY5BGYYYYJ.   J555YY557.              
.5B?.  :JBB~   !#5     !#5  ...PB~     ...^BG:....    ~:.   .7#5           :^^
7#5     .GB~   !B5     !B5     PB~        :BG.        .~7??J?JBG:      .~JPPPY
?#Y      PB~   !B5     !B5     PB~        :BG.       7GP7~^^^!BG:     ~5GY!:. 
^GB~    7BB~   ^BG.   .YB5     5#7        :BB:       P#!     JBG:    ^GG7     
 ^5G5JJYJPB~    JBP???YYB5     ^5GYJJ?.    7GPJ???.  ~PGJ77?5J5B!    JG5      
   .^~^..GB:     :~!!~. ^^       :~~~~      .^~~~~    .^!!!~. .^:    JG5      
 .?!^^^!5G7                                                          YB5      
 .!?JJJ?!:                                                           75?      
  </br>
terminal based cross-platform git client 
made with ♡ by @asinglebit
  </pre>
</div>

![2](https://github.com/user-attachments/assets/177dbf13-b9ad-480e-a1be-71a333454a44)

### Demo

Heres a recording of me going through the features of v0.1.12
[https://www.youtube.com/watch?v=oERA8MYlHjQ](https://www.youtube.com/watch?v=oERA8MYlHjQ)

### Disclaimer

I work on `guita╭` in my spare time, and give priority to the features I need in my day-to-day life. Use it with caution, and feel free to report issues or even better - contribute improvements! Im a lazy dude, and since this is a hobby project I just put unwrap() everywhere, such is life.

### Motivation

I needed a git client that would make it easy for me to understand where I am topologically at any given point in time. I also wanted it to be terminal based and cross-platform. I needed it to be fast. I also wanted to learn rust. So this is the project i picked to meet all of these goals at the same time.

### Features

- **Beautiful graph rendering** – Visualize commit history clearly.  
- **Reloading** – Reload the client manually using the shortcut when needed. Doesn't watch the directory.  
- **Immediate jumps** – Move through history without waiting.  
- **Pure TUI experience** – Ratatui based rendering.
- **Auth** – Currently simply attaches to the running ssh agent.  
- **Built-in diff viewer** – Inspect changes without leaving the terminal, however its very rudimentary.  
- **Tag management** – Create, view or remove tags.  
- **Stash management** – Create, view or remove stashes.  
- **Cherrypicking** – Happy path only for now.  
- **Opinionated** – Fetches prune branches and pull tags. Pushes are always hard and push local tags.
- **Keymap** – Keymap is completely customizable and is serilazied into `~/.config/guitar` folder (depending on your OS).
- **Heatmap** – Render a github-style heatmap of the repository.
- **Layout** – Somewhat primitive, also serilazied into `~/.config/guitar` folder (depending on your OS).
- **Terminal-friendly colors** – Easy on the eyes for long coding sessions with three builtin themes. 

### Planned features

- **Rebasing** – Ability to rebase.  
- **Merge** – Ability to merge.  
- **Conflicts** – Issue an alert with conflicting files to resolve externally.  

### Maybe I get to it someday features

- **Auth** – Comprehensive auth management.
- **Keymaps** – In-app keymap configuration.
- **Themes** – Custom themes.
- **Recent repos** – List of most recent repositories.

### Default keyboard mappings (they suck for now)

<div align="center">
<pre>
╭─────────────────────────────────────────────────────────────────────╮
│ [_]   [_][_][_][_] [_][_][_][_] [_][_][_][_] [_][_][_] [_][_][_][_] │
│                                                                     │
│ [_][_][_][_][_][_][_][_][_][_][_][_][_][___] [_][_][_] [_][_][_][_] │
│ [__][_][_][_][_][_][_][_][_][_][_][_][_][_ │ [_][_][_] [_][_][_][ | │
│ [___][_][_][_][_][_][_][_][_][_][_][_][_][_│           [_][_][_][_| │
│ [_][_][_][_][_][_][_][_][_][_][_][_][______]    [_]    [_][_][_][ | │
│ [__][_][__][_____________________][__][_][_] [_][_][_] [____][_][_| │
╰─────────────────────────────────────────────────────────────────────╯

shortcuts / normal mode:

Focus Next Pane                                                     Tab
Focus Next Pane                                                   Right
Focus Next Pane                                                 Char(l)
Focus Previous Pane                                     Shift + BackTab
Focus Previous Pane                                                Left
Focus Previous Pane                                             Char(h)
Select                                                            Enter
Back                                                                Esc
Minimize                                                        Char(.)
Toggle Branches                                                 Char(1)
Toggle Tags                                                     Char(2)
Toggle Stashes                                                  Char(3)
Toggle Status                                                   Char(4)
Toggle Inspector                                                Char(5)
Toggle Shas                                                     Char(6)
Toggle Settings                                                    F(1)
Action mode                                              Ctrl + Char( )
Exit                                                            Char(q)
Exit                                                     Ctrl + Char(c)
Page Up                                                          PageUp
Page Down                                                      PageDown
Scroll Up                                                       Char(k)
Scroll Down                                                     Char(j)
Scroll Up                                                            Up
Scroll Down                                                        Down
Scroll Up Half                                     Ctrl + Alt + Char(k)
Scroll Down Half                                   Ctrl + Alt + Char(j)
Scroll Up Half                                          Ctrl + Alt + Up
Scroll Down Half                                      Ctrl + Alt + Down
Go To Beginning                                                    Home
Go To End                                                           End
Scroll Up Branch                                         Ctrl + Char(k)
Scroll Down Branch                                       Ctrl + Char(j)
Scroll Up Branch                                              Ctrl + Up
Scroll Down Branch                                          Ctrl + Down
Scroll Up Commit                                          Alt + Char(k)
Scroll Down Commit                                        Alt + Char(j)
Scroll Up Commit                                               Alt + Up
Scroll Down Commit                                           Alt + Down
Find                                                     Ctrl + Char(f)
Solo Branch                                                     Char( )
Toggle Branch                                                   Char(t)
                                                                       
shortcuts / action mode:                                                  
                                                                       
Focus Next Pane                                                     Tab
Focus Previous Pane                                     Shift + BackTab
Focus Next Pane                                                   Right
Focus Previous Pane                                                Left
Select                                                            Enter
Back                                                                Esc
Minimize                                                        Char(.)
Toggle Branches                                                 Char(1)
Toggle Tags                                                     Char(2)
Toggle Stashes                                                  Char(3)
Toggle Status                                                   Char(4)
Toggle Inspector                                                Char(5)
Toggle Shas                                                     Char(6)
Toggle Settings                                                    F(1)
Exit                                                            Char(q)
Exit                                                     Ctrl + Char(c)
Page Up                                                          PageUp
Page Down                                                      PageDown
Scroll Up                                                            Up
Scroll Down                                                        Down
Scroll Up Half                                          Ctrl + Alt + Up
Scroll Down Half                                      Ctrl + Alt + Down
Go To Beginning                                                    Home
Go To End                                                           End
Drop                                                            Char(y)
Pop                                                             Char(t)
Stash                                                           Char(e)
Fetch All                                                       Char(f)
Checkout                                                        Char(c)
Hard Reset                                                      Char(h)
Mixed Reset                                                     Char(m)
Unstage                                                         Char(u)
Stage                                                           Char(s)
Commit                                                          Char(a)
Force Push                                                      Char(p)
Create Branch                                                   Char(b)
Delete Branch                                                   Char(d)
Tag                                                             Char(/)
Untag                                                           Char(?)
Cherrypick                                                      Char(])
Reload                                                          Char(r)

</pre>
</div>

### Releases

Please check the releases for the latest versions: https://github.com/asinglebit/guitar/releases

### Build yourself

Clone the repo and build with Cargo:

```bash
git clone https://github.com/asinglebit/guitar.git
cd guitar
cargo build --release
```

Your binary path after a successfull build:

```bash
guitar/target/release/guitar
```

Copy it wherever you want and run with a path to repo you wish to inspect

```bash
guitar ../path/to/your/repo
```

Or alternatively, alias the executable and then call it from a repo folder directly.

Running it from a non repo folder will crash the process. I will fix it sometime later...

### Screenshots
![1](https://github.com/user-attachments/assets/37df457b-bbf4-4d51-a965-c300b426cb62)
![6](https://github.com/user-attachments/assets/2ed8f14e-193b-4815-b37e-283bd129787f)
![5](https://github.com/user-attachments/assets/15e4630f-a141-4724-9d35-1b8601006598)
![4](https://github.com/user-attachments/assets/10389ec5-6780-4bcb-85dc-67f9e012ed63)
![3](https://github.com/user-attachments/assets/a408af0c-ef85-4692-914b-81562d3873e4)
