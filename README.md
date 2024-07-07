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

AES finite field uses the characteristic 2 with 256 elements: $GF(2^8)$, which employes the following irreducible polynomial:

$$
x^8 + x^4 + x^3 + x + 1
$$


## Top-down Overview

## S-boxes

## Key expansion algorithm

## Operations and transformations

## References


<script type="text/x-mathjax-config">
    MathJax.Hub.Config({
      tex2jax: {
        skipTags: ['script', 'noscript', 'style', 'textarea', 'pre'],
        inlineMath: [['$','$']]
      }
    });
</script>
<script src="https://cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS-MML_HTMLorMML" type="text/javascript"></script>