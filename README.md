# rusttar

This project is to implement the basic TAR format in Rust as specified by
[GNU](http://www.gnu.org/software/tar/manual/html_node/Standard.html).  Also
[wikipedia](https://en.wikipedia.org/wiki/Tar_(computing)#File_format) has a
really good reference on the format.

To start out, it will only implement the list function of tar.  It will take
an already created archive and read the data structures to list all of the
files.  Once that is working, I will go back and add in the create and untar
functions.

A tar archive is a sequential file format with each file stored in it given a
header and stored as-is.  The header contains metadata about the file:

- File name
- File mode
- Owner's UID
- Group's GID
- File size **in Octal**
- Last modified in seconds since Unix epoch **in Octal**
- Checksum for the header
- File type flag
- Linked file name (if it's a linked file)
- The string "ustar\0"
- UStar version of "00"
- Owner user _name_
- Owner group _name_
- Device major number
- Device minor number
- Filename prefix

tar can store many different file types.  The type of file is one of the fields
in the header.  It is stored as a single character:

- Normal file: '0' or '\0'
- Hard link: '1'
- Symbolic link: '2'
- Character special: '3'
- Block special: '4'
- Directory: '5'
- FIFO: '6'
- Contiguous file: '7'
- Global extended header with metadata: 'g'
- Extended header with metadata for the next file in the archive: 'x'
- Vendor extension: 'A'-'Z'

