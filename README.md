# rustrsc
rustrsc is an emulator for the RSC architecture written in Rust.

# How do I use it?

Simply download the binary and pass the binary command line arguments. In the future, there will be a GUI.<br>
There are two options, running the given microcode file or assembling it into a logisim accepted format.<br><br>


**Running your given microcode file in the emulator**

``rustrsc run microcode.txt``

**Using the in-built assembler to generate logisim formatted bytecode**

``rustrsc assembler microcode.txt output.txt``
