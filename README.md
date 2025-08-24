# PE\_parser

This repository hosts PE‑Parser, a learning‑first exercise in Rust’s spirit—focused, explicit, and safe—to demystify the Windows PE (Portable Executable) file format.




\*\*PE‑Parser\*\*: an educational parser for PE32 and PE32+ files.  



 \*\*Purpose\*\*: Learn how PE files are laid out and parsed, by explicitly defining each structure and parsing step—like writing safe, structured Rust code for binary formats.  


\*\*Parses\*\*:
 - DOS header  
 - Rich header (if present)  
 - NT headers  
 - Section headers  
 - Data directories → Import Table, Base Relocations, etc.  

 \*\*Highlights\*\*:

 - Precise modeling of binary structures  

- Step-by-step parsing functions  

 - Clear modular design ready to be ported to Rust  



 \*\*Getting Started\*\*:

 1. Clone the repo  

 2. Build: `cargo build` (if ported to Rust) or compile with C++  

 3. Run: `./PE-Parser path\\to\\binary.exe`  



 \*\*Dive Deeper\*\*: Read the accompanying blog post for lab-style walkthrough—detailing structure definitions, parsing logic, RVA resolution, and more. :



---




