/*
There is a "single" set of 18 instructions, repeated 14 times. The only things which changes are 3 literals which
I call m, n and o.
On the right is the successive result of each step, starting at w,x,y,z and with the input called i.

below, some expressions are abbreviated:
- q0 is z if m is 1, or floor(z / 26) if m is 26
- q1 is 1 if (z%26)+n is equal to i, else 0
- q2 is 1 if q1 is 0, else 0

inp w               i,  x,        y,               z
mul x 0             i,  0,        y,               z
add x z             i,  z,        y,               z
mod x 26            i,  z%26,     y,               z
div z m             i,  z%26,     y,               q0
add x n             i,  (z%26)+n, y,               q0
eql x w             i,  q1,       y,               q0
eql x 0             i,  q2,       y,               q0
mul y 0             i,  q2,       0,               q0
add y 25            i,  q2,       25,              q0
mul y x             i,  q2,       25*q2,           q0
add y 1             i,  q2,       (25*q2)+1,       q0
mul z y             i,  q2,       (25*q2)+1,       (25*q0*q2)+q0
mul y 0             i,  q2,       0,               (25*q0*q2)+q0
add y w             i,  q2,       i,               (25*q0*q2)+q0
add y o             i,  q2,       i+o,             (25*q0*q2)+q0
mul y x             i,  q2,       (i*q2)+(o*q2),   (25*q0*q2)+q0
add z y             i,  q2,       (i*q2)+(o*q2),   (25*q0*q2)+(i*q2)+(o*q2)+q0

Note that the new z at each stage depends only on the previous z, not the previous w, z or y.
And it is computed, from the old z and the current values of m, n, o and i - i being the current input
digit - as follows:

- if m = 1, then z' (new z) is given in terms of z by one of the following 2 recurrence relations:
a) z' = z
b) z' = 26z + i + o
- if m = 26, then the same 2 hold, under the same circumstances, with z replaced by floor(z/26)
Of the two cases, a) occurs if and only if i = z%26 + n.

i is always in the range 1..9, while o is always non-negative (and only zero once).
n though can be positive OR negative. Specifically the successive triples (m, n, o) are:

1. (1,13,10)
2. (1,11,16)
3. (1,11,0)
4. (1,10,13)
5. (26,-14,7)
6. (26,-4,11)
7. (1,11,11)
8. (26,-3,10)
9. (1,12,16)
10. (26,-12,8)
11. (1,13,15)
12. (26,-12,2)
13. (26,-15,5)
14. (26,-12,10)

There are up to 4 cases at each step, depending on:
- whether m is 1 or 26. If 1, z is either unchanged or increases, but if 26, it either:
- gets divided by 26, dropping any non-integral part (this is case a)
- drops any part that isn't divisible by 26 (without getting divided), and adding back on i and o. (this is case b)

It is easily shown that z MUST increase at some steps - step 1, for example (becoming i1 + 10, from 0).
So it must decrease by enough, at those where it does decrease.
Case b above (for m=26) may decrease z slightly. But it can be by at most 24 - o.
(25 is the most that can be removed, but at least 1 is always added on. And o is always added on.)
But case a removes (approx.) 25/26 of z, which is a "big deal", especially if it happens multiple times.

What determines - particularly in terms of digits - whether we have an increase or decrease at each step.
Broadly, the situation is:
m = 1, case a) - no change
m = 1, case b - large increase in z (25*z + i + o added on, ie. roughly multiplied by 26)
m = 26, case a) - large decrease (shrinks to approx 1/26 of size)
m = 26, case b) - little change, can be small increase or decrease

so broadly, we want m=26 case a to outweigh m=1 case b, or at least be roughly the same number.
Note that there are equal numbers of m=26 and m=1, but all the m=1s come first,
so there has to be an increase early.

And case b (the "increase" case, at least for m = 1) seems to be much more likely than a.
At some steps we have a choice of digits that allow a, but at others b is forced.
It would be good to understand that situation!

Recall that the condition is that z%26 + n = i. This is ruled out in many cases because either the
previous z (mod 26), and/or n itself, are too big. In particular, the absolute maxium n can be here is 9
(and that assumes the previous z is a multiple of 26). But ALL positive ns are over 9. The only n's below 9
are the negative ones, which are at steps 5,6,8,10,12,13,14. Encouragingly, these are EXACTLY the steps for which
m = 26!

Those are 7 steps, and on the other 7, we MUST be in case b with m = 1 - the "worst" case in terms of increase
So the best - and likely only - way to get enough of a decrease, is to ensure that ALL of these steps go to
"case a".

By careful analysis of these, and keeping track of the expression for z at each stage in terms of the existing
digits, we find that the output of 0 occurs when precisely the following conditions all hold (here i1 denotes
the first digit of the 14-digit number, and so on):
-- i5 = i4 - 1
-- i6 = i3 - 4
-- i7 = 1
-- i8 = 9
-- i10 = i9 + 4
-- i12 = i11 + 3
-- i13 = i2 + 1
-- i14 = i1 - 2

meaning the 14-digit numbers which work are all of the form:
abcd(d-1)(c-4)19e(e+4)f(f+3)(b+1)(a-2)
for a-f such that all 14 digits are in the range 1-9.

This makes it easy to compute the highest and lowest valid values - they are simply "hardcoded" below!
*/

pub fn part_1() -> u64 {
    98998519596997
}

pub fn part_2() -> u64 {
    31521119151421
}
