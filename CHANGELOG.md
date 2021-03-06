#### 2020.03.03
- Context menu and Inspect window for entities
  ![img](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/e9666a346977af94e537b6bb87ea7615/rustarok_inspect.jpg)
  ![img](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/9cfc3e00935bfd14fae65ec857d3a4f7/rustarok_inspect2.jpg)

#### 2020.01.04
- Improving netcode
  ![img](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/99cc03628e1f556539c380e4e7e3dba3/netcode_works.gif) 

#### 2019.12.27
- Experimental: Upscaling sprites for better quality

#### 2019.12.23
- Handling window size
- Many systems and component on the Client were refactored to be Singleton (Input Handler, Renderer, Audio etc)

#### 2019.12.22
- Debug visualization for net code
    - Displaying the acknowledged position of characters
    - Displaying when a rollback happens (the "ghost" character blinking in red)  
    ![img](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/e075a6a2fcbcaff08f9b57d1906ffde1/moving_lag_rollback3.gif) 

#### 2019.12.20
- Initial client and server code separation (client, server, common)
- Basic client side prediction
- First working netcode
  ![img](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/09a580a89ea26f3ff0ce50bdace71769/first_network.gif) 

#### 2019.11.26
- Statuses are now simple enums and not Boxed traits. 
    - no malloc
    - no ugly hacky reflection
    - easier serialization
- Remove websocket and BrowserClient functionality (performance was not sufficient, so usual netcode will be implemented) 
#### 2019.11.15
- Added cylinder effect to Heal  
![img](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/c67f321d831cfb317c1f95558e584cce/Peek_2019-11-15_21-51.gif)

#### 2019.11.14
- Barricade cannot be put onto an already occupied tile
- `finish_cast` now expects a `FinishCast` struct as a parameter

#### 2019.11.13
- `set_resolution` and `set_fullscreen` commands
- `resolution_w` and `resolution_h` configs in config.toml

#### 2019.10.27
- Startup time optimization
    - Ground is loaded on a second thread as well
    - debug startup: ~5s
    - release startup: ~1.9s
    
#### 2019.10.27
- Startup time optimization  
Sprites and models are loaded on a background thread asynchronously. 
    - old, debug mode: 24s
    - old, release mode: ~3.5s (max value 6.1s)
    - new, debug mode: 7.6s
    - new, release mode: 2.7s (fairly stable, no fluctuations)

#### 2019.10.24
- Red and blue colors for all classes  
![Palettes](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/c22dd3a7eda670ad6b1268ff12697d54/image.png)
- added `init.cmd`: A script file whose lines are executed on startup via the console system.  
It makes it possible to bind commands to shortcuts (see next point)
- Key binding command, e.g.: ``bind_key alt+Num1 toggle_console``
- `KeyState`s in `HumanInputComponent` are stored in a fixed size array not a hashmap (the index is the scancode value, which is a value from 0 to 284, hashmap was unnecessary)
- ``config-runtime.toml`` were expanded with `execute_script` property. It is for executing more complex and multiline commands (e.g. for the screenshot above, I needed to call `set_job JOB_NAME` and `clone` commands for all the available classes).  
Each commands are executed in a single frame to avoid problems with the physics system.  
Will be removed soon because it is a quite hacky solution.

#### 2019.10.23
- Palettes  
![Palettes](https://trello-attachments.s3.amazonaws.com/558a94779b3b3c5d89efeaa6/5d3dad963f865934aa69f051/2e4b89ed1f83638bc885f9ee0bf215ef/image.png)
