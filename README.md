# Aleph1. Goofy "Hashing".
Alelph1 is a system and a pseudohashing algorithm written in Rust for educational purposes.
Do Not use it. Its goofy.
## How it works?
1. Get Plain Text length(X)
2. Get Salt Length(Y)
3. Get a random number user provide(Z)
4. Generate a salt.
5. (X * X) - Y + Z;
6. If result is bigger than 70, Subtract 1 until it is 70 or less;
7. Use this number as an index for character list then push the resulting character to a vector.
8. Turn it into a string and combine it with salt