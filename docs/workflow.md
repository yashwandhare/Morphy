## worflow 
```
$ filex
↓
ASCII ART + welcome
↓
"What do you want to convert?"
  ▸ Image
  ▸ PDF
  ▸ Video
↓
"Select a file"
  ▸ Native file picker OR CLI navigator
↓
File extension auto-detected
↓
"Convert to:"
  ▸ options filtered by input type
↓
Progress bar + minimal logs
↓
Saved to original directory
↓
Success message
```
## project strcture - 
```
filex/
 ├── app.py
 ├── ui/
 ├── core/
 ├── converters/
 ├── utils/
 └── docs/
```
1. app.py - what happens when user calls the project the main menu of the app 
2. ui/ - the TUI of the project 
  a. slpash.py - greetings 
  b. menus.py - lets user choose which type of file to convert
  c. filepicker.py - lets user choose a file to convert 
3. core/ - brain of project 
  a. detect.py - detects what kind of file is selected to extract extension
  b. registry.py - what type of conversion to choose based on extension extracted
4. converters/ = recoives x input produces y output
5. utils/  - misc files 
6. docs/ - project documentation
