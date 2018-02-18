Name: Grant Lindberg
Email: grant.lindberg@wsu.edu
Due date: 02/21/2018

Project: wsu_crypt
Description: A custom encryption/decryption program that utilizes methods from TwoFish and Skipjack.
This program only works with 64-bit inputs, and it does not do padding.
You must have Rust installed in order to build and run this program.

How to install:

Option 1:

    Installing rust:

    1. Rust can be installed via executable at: https://www.rust-lang.org/en-US/install.html
    2. In UNIX: curl https://sh.rustup.rs -sSf | sh
    3. Other alternatives: https://www.rust-lang.org/en-US/other-installers.html

Option 2:

    Demoing rust:

    1. Email me and I will be happy to make the time to meet you on campus
    2. I am also open to using skype, discord, etc. if need be

How to build/run:

    1. cd wsu_crypt/
    2. cargo build (builds the executable) *This step can be skipped
    3. cargo run encrypt (builds and runs the executable.
                          Uses plaintext.txt and key.txt for input and writes to stdout)
    4. cargo run decrypt (builds and runs the executable.
                          Uses ciphertext.txt and key.txt for input and writes to stdout)

    NOTES:
    1. You must have a file under the 'input/' directory entitled 'key.txt' and another file entitled 'plaintext.txt' for encryption to work.
    2. You must have a file under the 'input/' directory entitled 'key.txt' and another file entitled 'ciphertext.txt' for decryption to work.

Running tests:

If you wish to run the unit tests I have been using in my program, execute 'cargo test'.
Note that the two failing tests are related to decryption. It is still not clear to me why they fail, because they seem to have the correct output.

Included files:

1. main.rs - Handles input from the command line, en/decrypts as specified, and prints the result to the terminal.
2. convert_types.rs - Converts vectors of some type to vectors of different types. Converts blocks of some type to blocks of other types.
3. crypt.rs - En/decrypts as specified by the user and returns the result. Accesses the F() and G() functions, the F table, etc.
4. subkey_gen.rs - Generates the subkeys needed for both encryption and decryption.
5. test.rs - Contains all unit tests involved in the making of this program.
6. whiten.rs - Handles whitening for input and output for en/decryption.
