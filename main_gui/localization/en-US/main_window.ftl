# Windows menu entries. These should use & before keyboard accelerator letters as they will
# be interpreted by the Windows API. The sequence '{"\u0009"}' is equivalent to "\t" in C, and
# allows adding a right-aligned text section to menu items (often used to display keyboard
# shortcuts).
menu-file = &File
menu-file-new = &New..
menu-file-open = &Open{"\u0009"}Ctrl+O
menu-file-save = &Save{"\u0009"}Ctrl+S
menu-file-save-as = Save &As...{"\u0009"}Ctrl+Shift+S
menu-file-exit = E&xit{"\u0009"}Alt+F4
    
menu-edit = &Edit
menu-edit-cut = C&ut{"\u0009"}Ctrl+X
menu-edit-copy = &Copy{"\u0009"}Ctrl+C
menu-edit-paste = &Paste{"\u0009"}Ctrl+V

menu-windows = &Windows

menu-help = &Help
menu-help-about = &About { $kanaya-brand-name }