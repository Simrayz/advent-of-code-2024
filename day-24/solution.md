```
z~n~ = a~n~ XOR b~n~
a~n~ = x~n~ XOR y~n~
b~n~ = (a~n-1~ AND b~n-1~) OR (y~n-1~ AND x~n-1~)
```

x11 AND y11 -> z11
x11 XOR y11 -> vbj

=z11,vbj

pwp OR cwc -> z24
(
qvs AND kmc -> cwc # This one first
x24 AND y24 -> pwp # This one third
)
?

vsb AND dkp -> z38
dkp XOR vsb -> hqh

=z38,hqh

tcg OR fbd -> z45
(
wrg AND mjr -> tcg # This one first
y44 AND x44 -> fbd # This one second
)

z11,vbj,z24,cwc,z38,hqh,z45,tcg

z11,vbj,z24,cwc,z38,hqh,z45,tcg
// Sort the list above
hqh,fbd,pwp,vbj,z11,z24,z38,z45
// Next to try
hqh,pwp,tcp,vbj,z11,z24,z38,z45

---

z24 = pwp OR cwc
pwp = x24 AND y24
cwc = qvs AND kmc
z24 = (x24 AND y24)=pwp OR (qvs AND kmc)=cwc
