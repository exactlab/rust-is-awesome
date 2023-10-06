# Slices

Simple examples to showcase the use and usefulness of the slices in rust.  
Here we used slices to write a function that accept different types of containers holding i32s.  
This is quite nice considering that we did not have to use templates or other sort of generic programming.
It works simply because the compiles knows (because it is told) how to convert container types into slices. 

### For C++ coders
Basically slices are to rust what ranges are to C++. 
The big difference being that slices are built into the rust language.

```bash
cargo run 
```

