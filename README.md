# Implementing AES in Rust from scratch

## Mathematical basics

### Galois Field

Galois field is also known as finite field, which is an object
in abstract algebra that deals with finite structures. Size of
Galois field is determined by prime number $p$ and is denoted
as $GF(p)$ or $F_p$. There are 2 operations that are performed in finite
field: addition and multiplication, which follow specific rules.

#### Addition

Addition is performed by adding two of polynomials together, 
and reducing the result modulo the characteristic. In finite field
with characteristics 2 $GF(2^n)$, addition can be performed as XOR operation. Symbol $\oplus$ or $+$ maybe used to denote addition in finite field.

| **Polynomial**  | $(x^6+x^4+x+1) + (x^7+x^6+x^3+x) = x^7 + x^4 + x^3 + 1$ |
|:---------------:|---------------------------------------------------------|
|   **Binary**    | ${01010011} + {11001010} = {10011001}$                  |
| **Hexadecimal** | ${53} + {CA} = {99}$                                    |


#### Multiplication

Multiplication is multiplication modulo an irreducible polynomial used to define finite field. Symbol $\times$ or $\cdot$ maybe used to denote multiplication in finite field.

For example: $a = x^2 + x + 1 \ (7 - 111b),\ b = x + 1 \ (3 - 011b)$ and primitive polynomial $x^4+x+1$ in $GF(2^4)$.

$$
a \cdot b = (x^2 + x + 1) \cdot (x+1) = x^3 + x^2 + x^2 + x + x + 1 = x^3 + 1
$$

Since the power of multiplication is less than $x^4$, there is no need of performing modulo operation.



#### Properties

1. Finite size - finite filed has specific size, which can't be changed, size of field is determined by $p$.
2. Closure - result of operation (addition or multiplication) will always be an element of determined field.
3. Commutative - order of elements does not matter in performing operations. For example, $a+b = b+a$ and $ab = ba$
4. Associative - grouping of elements in an operation does not matter. For example, $(a+b) + c = a + (b+c)$ and $(ab) \cdot c = a \cdot (bc)$
5. Distributive - multiplication distributes over addition $a \cdot (b+c) = a\cdot b + a\cdot c$
6. Identity elements - Galois field has 2 identity elements: $0$ for addition and $1$ for multiplication.
7. Inverse elements - every element has inverse element under addition and multiplication operations. The inverse element for addition is the negative of the original element, and the inverse element for multiplication is the reciprocal of the original element.

### Rijndael's Finite Field

AES finite field uses the characteristic 2 with 256 elements: $GF(2^8)$, which employs the following irreducible polynomial:

$$
x^8 + x^4 + x^3 + x + 1
$$


## SP-network

Substitution-permutation network is a series of linked mathematical operations used in block cipher. This sturcture
introduces *confusion* and *diffusion*. Confusion means that input undergoes complex transformation and the relationship between key and ciphertext is being hidden. Confusion in SP-network is provided by S-boxes (substitution boxes). Diffusion means that if we change a single bit of the plaintext, then about half of the bits in the ciphertext should change, and similarly, if we change one bit of the ciphertext, then about half of the plaintext bits should change. The purpose of diffusion is to hide the statistical relationship between the ciphertext and the plain text. In SP-network diffusion is provided in permutation player by performing some linear transformations (e.g. mixing rows, columns), however non-linear transformations can be used as well.


![](https://www.researchgate.net/profile/Liam-Keliher-2/publication/2822741/figure/fig2/AS:669553994502155@1536645520528/Example-SPN-with-N-16-n-M-4-R-3.png)



## S-boxes

Substitution boxes are basic components of symmetric key algorithms, which performs substitution. S-boxes introduces non-linearity in encryption algorithm. S-box takes input $m$ a transforms it to $n$, where $m \neq n$. For AES encryption, s-box
looks like this:

|      |00|01|02|03|04|05|06|07|08|09|0a|0b|0c|0d|0e|0f|
|------|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|**00**|63|7c|77|7b|f2|6b|6f|c5|30|01|67|2b|fe|d7|ab|76|
|**10**|ca|82|c9|7d|fa|59|47|f0|ad|d4|a2|af|9c|a4|72|c0|
|**20**|b7|fd|93|26|36|3f|f7|cc|34|a5|e5|f1|71|d8|31|15|
|**30**|04|c7|23|c3|18|96|05|9a|07|12|80|e2|eb|27|b2|75|
|**40**|09|83|2c|1a|1b|6e|5a|a0|52|3b|d6|b3|29|e3|2f|84|
|**50**|53|d1|00|ed|20|fc|b1|5b|6a|cb|be|39|4a|4c|58|cf|
|**60**|d0|ef|aa|fb|43|4d|33|85|45|f9|02|7f|50|3c|9f|a8|
|**70**|51|a3|40|8f|92|9d|38|f5|bc|b6|da|21|10|ff|f3|d2|
|**80**|cd|0c|13|ec|5f|97|44|17|c4|a7|7e|3d|64|5d|19|73|
|**90**|60|81|4f|dc|22|2a|90|88|46|ee|b8|14|de|5e|0b|db|
|**a0**|e0|32|3a|0a|49|06|24|5c|c2|d3|ac|62|91|95|e4|79|
|**b0**|e7|c8|37|6d|8d|d5|4e|a9|6c|56|f4|ea|65|7a|ae|08|
|**c0**|ba|78|25|2e|1c|a6|b4|c6|e8|dd|74|1f|4b|bd|8b|8a|
|**d0**|70|3e|b5|66|48|03|f6|0e|61|35|57|b9|86|c1|1d|9e|
|**e0**|e1|f8|98|11|69|d9|8e|94|9b|1e|87|e9|ce|55|28|df|
|**f0**|8c|a1|89|0d|bf|e6|42|68|41|99|2d|0f|b0|54|bb|16|

For example byte `0x42` will be mapped to `0x2c`. Every 8-bit value is mapped to different 8-bit value. For AES decryption, s-box is defined in the following way:

|      |00|01|02|03|04|05|06|07|08|09|0a|0b|0c|0d|0e|0f|
|------|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|**00**|52|09|6a|d5|30|36|a5|38|bf|40|a3|9e|81|f3|d7|fb|
|**10**|7c|e3|39|82|9b|2f|ff|87|34|8e|43|44|c4|de|e9|cb|
|**20**|54|7b|94|32|a6|c2|23|3d|ee|4c|95|0b|42|fa|c3|4e|
|**30**|08|2e|a1|66|28|d9|24|b2|76|5b|a2|49|6d|8b|d1|25|
|**40**|72|f8|f6|64|86|68|98|16|d4|a4|5c|cc|5d|65|b6|92|
|**50**|6c|70|48|50|fd|ed|b9|da|5e|15|46|57|a7|8d|9d|84|
|**60**|90|d8|ab|00|8c|bc|d3|0a|f7|e4|58|05|b8|b3|45|06|
|**70**|d0|2c|1e|8f|ca|3f|0f|02|c1|af|bd|03|01|13|8a|6b|
|**80**|3a|91|11|41|4f|67|dc|ea|97|f2|cf|ce|f0|b4|e6|73|
|**90**|96|ac|74|22|e7|ad|35|85|e2|f9|37|e8|1c|75|df|6e|
|**a0**|47|f1|1a|71|1d|29|c5|89|6f|b7|62|0e|aa|18|be|1b|
|**b0**|fc|56|3e|4b|c6|d2|79|20|9a|db|c0|fe|78|cd|5a|f4|
|**c0**|1f|dd|a8|33|88|07|c7|31|b1|12|10|59|27|80|ec|5f|
|**d0**|60|51|7f|a9|19|b5|4a|0d|2d|e5|7a|9f|93|c9|9c|ef|
|**e0**|a0|e0|3b|4d|ae|2a|f5|b0|c8|eb|bb|3c|83|53|99|61|
|**f0**|17|2b|04|7e|ba|77|d6|26|e1|69|14|63|55|21|0c|7d|


## Key expansion algorithm

## Operations and transformations

## References

