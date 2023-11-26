# RUST-Generic-Dynamic-Array
A generic dynamic array (also called vector) implementation done from scratch by me.


This implementation uses a pointer to memory, number of elements of the specified type, and size of memory block in terms of number of storable elements to the provided type.


Every time the complete allocated memory block is filled, double new memory is allocated and old memory is released. When the number of elements goes equal or below 1/3 of the allocated memory, half new memory is allocated and old memeory is released.


This implementation also releases memory when the array goes out of scope.

Functions implemented: 
=> length

=> get

=> set

=> push

=> pop

=> insert at

=> remove at

=> copy

=> print array for displayable types

=> find item for comparable types

=> remove item for comparable types
