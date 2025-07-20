# A simple calculator written with Rust.
How to install and run: Download the .zip file by going to Code > Download ZIP and extract the zip or run

```git clone https://github.com/praneethashok14/CalCLI_RUST```

Then type cd <CalCLI_DIR>. Replace <CalCLI_DIR> with the folder where CalCLI is located. 
Then install Rust with the instructions on this link: 

https://www.rust-lang.org/tools/install. 

Also install make if not installed. More info on installing make on make-install.md. 
If you already have rust and make then you can run 

```make```

Then 

```make install```

for local installation and

```sudo make install```

for system wide installation.
Then add the installation folder to your $PATH with

```echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc``` or ~/.bashrc if you are on bash

for user installation and

```echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc``` or ~/bashrc if you are on bash
for system installation.
Or if you're on Arch/Manjaro/EndeavourOS:
```yay -S calcli```
or
```paru -S calcli```
