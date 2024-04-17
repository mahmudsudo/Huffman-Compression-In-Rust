Huffman Compression in Rust
This is a simple showcase of Huffman compression implemented in Rust. Huffman coding is a widely used algorithm for lossless data compression. It assigns variable-length codes to input characters, with shorter codes assigned to more frequent characters, and longer codes assigned to less frequent characters.

How to Use
Clone the Repository:

bash
Copy code
git clone https://github.com/yourusername/huffman-compression-rust.git
Build the Project:

Navigate to the project directory and run:

bash
Copy code
cargo build --release
Compress a File:

To compress a file, run the following command:

bash
Copy code
cargo run --release -- compress <input_file> <output_file>
Decompress a File:

To decompress a previously compressed file, run:

bash
Copy code
cargo run --release -- decompress <compressed_file> <output_file>
Example
bash
Copy code
cargo run --release -- compress input.txt compressed.bin
cargo run --release -- decompress compressed.bin decompressed.txt
About Huffman Coding
Huffman coding works by constructing a binary tree of nodes. Each node represents either a character in the input data or an internal node representing the combined frequencies of its children. The algorithm builds this tree in a greedy manner, ensuring that more frequent characters have shorter codes.

Performance
This implementation aims for simplicity and clarity rather than optimal performance. However, it demonstrates the core concepts of Huffman coding and can serve as a starting point for further exploration or optimization.

Contributing
Contributions are welcome! Feel free to submit issues or pull requests if you have any suggestions for improvements or if you encounter any bugs.
