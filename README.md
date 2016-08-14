#Riceinator
Manage your configuration files easily with templating

##Installation
  1. ```git clone https://github.com/matthunz/riceinator.git```
  2. ```cd``` into the newly-created directory
  3. Run ```cargo build --release```
  4. Copy the executable in ```./targer/release``` to somewhere in your $PATH, like ```/usr/local/bin/```
  5. Make a new directory called riceinator under ```~/.config``` and copy the example files to it

##Usage
  1. Move your file, for example xresources, into ```~/.config/riceinator/templates```
  2. Make an entry for it inside config.toml with ```templatename = path/to/real/file```
  3. Replace any text inside to be changed with {{ variablename }} inside the template file
  4. Add the variable name under [variables] in config.toml
  5. Run the program!
