# Windows menu entries. These should use & before keyboard accelerator letters as they will
# be interpreted by the Windows API. The sequence '{"\u0009"}' is equivalent to "\t" in C, and
# allows adding a right-aligned text section to menu items (often used to display keyboard
# shortcuts).
menu-file = &File
    # .new = &New..
    # .open = &Open{"\u0009"}Ctrl+O
    # .save = &Save{"\u0009"}Ctrl+S
    # .save-as = Save &As...{"\u0009"}Ctrl+Shift+S
    # .exit = E&xit{"\u0009"}Alt+F4
    
menu-edit = &Edit
    # .cut = C&ut{"\u0009"}Ctrl+X
    # .copy = &Copy{"\u0009"}Ctrl+C
    # .paste = &Paste{"\u0009"}Ctrl+V

menu-windows = &Windows

menu-help = &Help
    # .about = &About { $kanaya-brand-name }