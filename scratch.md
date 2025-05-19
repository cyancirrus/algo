```
minimum substring
samestrings are the same 

f(s1, s2) {
    counter = 1;
    if char1 = char2 -> {
        ...iterate through the thing;
        potential = 1 + f(s1[1..], s2[1..),
    } else 
        potential = max (
            f(s1[1..], s2),
            f(s1, s2[..-1])
        }
    )
}

















```
buy and sell stock
profit = s[T] - s[t];
if i sell at [T] i can buy sell previous at window [t-T)





lets try to think top down recursion
if i have a lower potential price at t = n-1 = s(t*)
then
f(n) = f(n-1) + (s(N) - s(t*))
else
f(n) = f(n-1)
-- track potential min
-- track potential max
1..5.3..4.2..10

i see 4 and it causes me to execute
stock - potential_sell + potential_buy - potential_sell 

----
buy = 1
potential_sell = 5
potential_buy = 3
sell 4

=> execute
profit = 5-1
buy = 3
potential_buy = i32::max;
potential_sell = 4
=> iterate


wrapup
if potential_buy - potential_sell > 0
add








lets consider

1 ... 3 .. 10
if i buy
profit = 9, b,s at (0, e)
profit = 2 + 7 (0,m), (m, e)
a, b, c
p = b-a + c-b = c-a
=> buying and selling on same day is washed

more tricky

1..5..2..10
(1, 5), (2, 10)
... this feels like the water problem
is it the water problem 

101 <- 1, it's similar but we don't need it to be left capped






pascals binomial
```
n!
(n-k!) *k!

n*n-1..n-k
k!

n*n-1..n-k*n-k-1
(k+1)!

=> f(n,k+1) = f(n,k)* (n-k-1)/(k+1)


f(0, 2) = 1
f(1, 2) = f(0,0) * 2 / 2


```
