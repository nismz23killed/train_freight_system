# train_freight_system

Run the app using rust cargo run

ex:
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/train_freight_system`
Select options below
[N] Node input [ ex: N,A where A=name]
[E] Edge input [ ex: E,E1,A,B,30 where E1=name, A=node1, B=node2, 30=travel time]
[T] Train input [ ex: T,Q1,6,B where Q1=name, 6=Capacity, B=node location]
[P] Package input [ ex: P,K1,5,A,C where K1=name 5=Weight, A=node origin, B=node destination]
[X] deliver packages
[C] Clear data
[_]Any invalid keys will show the options
n,a
n,b
n,c
e,e1,a,b,30
e,e2,b,c,10
p,k1,5,a,c
t,q1,5,b
x
W=0, T=Q1, N1=B, P1=[], N2=A, P2 =[]
W=30, T=Q1, N1=A, P1=["K1"], N2=B, P2 =[]
W=60, T=Q1, N1=B, P1=["K1"], N2=C, P2 =[]
W=70, T=Q1, N1=C, P1=[], N2=, P2 =["K1"]
completed delivery in: Minute(70)
```
