```
coin change
- in our denomination you can greedy search
- you might not be able to greedy search given any denominations

count the number of ways to make change
hmmm
while amount > 0
    hmmm cc(amount - d[i], d[j])

-- want to prevent the number of duplicates
-- want to calculate all different ways

hmmm lets think of memoization we could build this up
something like - initialize a hashmap
while sum < amount
    sum over all inputs so far for every d[i] and then pull back the entry

- is okay but i don't like the way the data is stored
can we store this in a better way?
    - the data is not contigous


lets build with a hashmap and then we might see something pop out
--------------------
hmmm that's gross lets think of it another way

change(a0) = Sum :: change_(a0 - d[i])
okay this gives us a good starting place

----
problem 
(1 x 5), (5 x 1)
-> a[5] = 2
=>
c(10, [5,1]) = c(5, [5,1]) + c(9, [1])
- c(5, [5,1]) = 2
- c(9, [1]) = c(8, [1]) ... = c(5, [1]) = 2
=> 10

something is going wrong with the recursion it's not so simple
- maybe building up will be easier?
c(2) = Sum c(2 - d) 
-- same thing how do i kick out -- do they all need their own caches?
can we turn this into a matrix?
- lets make it so they all have their own cache and see what happens

- okay solution has been that instead of a singular cache and an iterate over all things we do something like
c(a, d, k) = sum [k..] change(a - d[k], d[k..])
-- this is recursive how can we build it up?

Likely need the same style of thing
- build up the first value
- then build up the second (can use the first values results)
- ie c(a-d[k]) = dp[k-1] + this thingy
- then just return the last value















```
unique paths with obstacles
likely not symetric ie p(i,j) != p(j, i);
- still appears like it might have the same recurrence relation ie

f(m,n) = f(m-1, n) + f(m, n-1)
... but there's an issue to where if it's included in robots then it's not valid
just looks like you need an if the obstacle exists ie somethign like

`
prev = ...
curr = ...

curr[j] = if grid[i,j] != 1 {
    prev[j] + curr[j-1]
} else {
    0
};

kk lets try i think this all reasons out 


```
unique paths

- seems similar to the steps in a way
- lets consider the bottom right

1 x 1 -> 1
2 x 1 -> 1
2 x 2 -> 2
--
what's different?
- there's a choice ie both i,j have a way to go
2 x 3 -> (right -> then solution 2x2, then 2 more)
so it's like
2 x 3 -> 2x2 +  1x2
2 x 4 -> 2x3 + 1x3

1 x n || n x 1 -> 1

# this is going to be a triangle no need for full i x j
4 x 2 == 2 x 4

f(m, n) {
    if n || m == 1 {
        return 1
    } else {
        return f(m-1, n) + f(m, n-1)
    }
}

-- what does f(2,2) look like
f(1, 2) + f(2,1) = 2

really we could ignore triangle for a second and have something like
sum +=
dp[i][j] and then when you calculate something store it
-- lets do a recursive and then investigate



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

- could i do this as a dp matrix?
dp ~ #s1 X #s2

s[i][j] == s1[i]=s2[j]
-- what's the decision point, how to i path through the data?
if s[i][j] == false -> then take take max of
s[i+1][j], s[i][j+1]
-> each thing path will be a diagonal
so it's something like
-> is this any different in it's calculation?
not really

- one pass to calculate the dp matrix
how can i parse the structure?

- lets write a recursive function and see what it looks like


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
