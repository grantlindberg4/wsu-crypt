Name: Grant Lindberg

Email: grant.lindberg@wsu.edu

Due date: 02/21/2018

Project: WSU Crypt

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

    1. cd project_1/
    2. cargo build (builds the executable)
    3. cargo run encrypt (both builds and runs the executable.
                          Uses plaintext.txt and key.txt for input and writes to stdout)
    4. cargo run decrypt (both builds and runs the executable.
                          Uses ciphertext.txt and key.txt for input and writes to stdout)

Running tests:

If you wish to run the unit tests I have been using in my program, execute 'cargo test'
