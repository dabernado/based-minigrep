# Pandion
Pandion is a search engine-like file manager for the terminal. It uses Vim-like keybindings, uses Rust and is radically concurrent. Most of its controls and UI is inspired by Ranger.

Pandion is currently a WIP.

## Why?
File managers are a slow and painful experience to use; you have to know where the file you want to edit is beforehand, and then press enter or click multiple times through directories to reach that file. For files of which you don't know the location of and are trying to find, that eats up even more time. Browsing through the filesystem on a computer is such a simple task, yet is very tedious, interrupts your workflow and eats up far more time than it should.

We got tired of this, and decided to build a file manager that acts more like a search engine, rather than a filesystem browser. If you know the name of the file you want to access, all you have to do is type it in and it will appear. If you don't know the file's name but know what it might be associated with, just type the keyword in and Pandion will query for you - for example, enter "dhcpcd" for dhcpcd-related libraries and configs, or enter "cattle" to search for text files that may contain the word "cattle".

## Features
	- Search for files based on name, extensions, keywords (for text files), size or path
	- File actions, such as moving or copying, can be done by editing their "Directories" list
