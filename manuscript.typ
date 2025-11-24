#let title = "Combinatorial Graph Theory Final Project"
#let due = datetime(year: 2025, month: 12, day: 9) 
#show link: set text(fill: blue)
#show link: underline

#set page(
  paper: "us-letter",
)
#set par(justify: true)
#set text(
  font: "Libertinus Serif",
  size: 11pt,
)

#align(center)[
  #stack(
    dir: ttb,
    spacing: 1em,
    text(14pt, title, weight: "bold"),
    text(12pt, "Jordan Bourdeau"),
    text(
      12pt,
      due.display(),
    ),
  )
]


= Problem Proposal & Background

It is common for instructors to incorporate group projects within their course curriculum. These may use a single fixed group over the length of a class, or assign new groups each time. The former of these approaches is boring from the combinatorics perspective, so we ignore it. A natural question for any instructor who loathes the idea of students experiencing repeated grouping to ask themselves is whether their group assignments are possible without duplicating groups over the course. This groundbreaking work solves the eons-old combinatorics problem. We start with the simple and common case of pairings, eventually working up to arbitrary group sizes. Specifically, we look to answer the following:

- Given a class of _n_ students, how many unique groups with at least 2 members can we construct before being forced to repeat group members?
  - How does the parity of _n_ (odd or even) affect this?
  - How does this change given some initial set of conflicts between students?
- How do these answers change when groups are of minimum size _k_, where _k_ is some integer $gt.eq 2$?
  

= Partner Matchings (k = 2)

We represent the set of students, $J$, as vertices in the $G$. A student can be placed into a group with any other student they share an edge with. After each assignment, the edges between every student in each group will be removed. A "conflicting" assignment occurs when two students are paired with each other but do not contain an edge between them. We call the set of students which student $j$ cannot be grouped at iteration $i$, $c_(j,i)$. How many times can this process be repeated before a conflicting assignment is guaranteed to occur? 

== Simple Case

The simplest case to consider is when the initial graph is $K_n$ (given $n$ students) where $n$ is even. Because the initial graph is complete, every edge can be treated the same. Since $n$ is a factor of the group size, 2, there will be no uneven groups. Thus, a valid and optimal pairing is for every vertex to be grouped with every other vertex over $n - 1$ rounds.

== Hopelessly Complex Case

When $n$ is odd, there must exist one non-conflicting group with size 3 and $(n - 3) / 2$ non-conflicting groups with size 2 in each iteration. Unfortunately, it is at this point where the problem already becomes incredibly difficult, and potentially NP-hard.

=== Similar Problem: Round-Robin Tournament

A commonly used strategy for similar matchmaking situations is the round-robin tournament method. This technique adds "dummy" members to round up to the nearest increment of the grouping size. A sample run for even group sizes and an odd number of teams is given below (note D represents the dummy, indicating a bye in the round):

#align(center)[
```
1 2 3 4
5 6 7 D

1 5 2 3
6 7 D 4

1 6 5 2
7 D 4 3

1 7 6 5
D 4 3 2

1 D 7 6
4 3 2 5

1 4 D 7
3 2 5 6

1 3 4 D
2 5 6 7
```
]

Because instructors are unlikely to give students a bye on an assignment due to parity of the class size, this won't fly. The lower bound on group size means we must add remaining students into an existing groups, thereby causing more edges to be deleted. 

== A Humble Upper Bound

When $k = 2$ and $n$ is odd, $(n + 3) / 2$ edges will be consumed per assignment. From this we can extract a loose upper bound on the number of possible iterations based on the maximum number of times this quantity of edges could be removed from the complete graph:

$ N_"edge_max" = floor((|E(K_n)|) / ((n + 3) / 2)) = floor((2 |E(K_n)|) / (n + 3)) $

Even with this highly constrained variant of the problem, it is difficult to establish a lower bound between then $gt.eq 1$ for a guaranteed number of iterations. This is due to the highly dynamic relationships between vertices in the graph. There is no global way to partition sets like in the Turán graph because each vertex has its own local view of which vertices could be valid partners. Thus, even if a given vertex has enough neighbors to create a group of size $k$, those neighbors may have some conflict with each other preventing it. For instance, consider $K_n$ with $n = 5$. The first iteration will select one $K_3$ and one $K_2$ whose edges to remove. Afterward, there are no 3 vertices without conflicts to create the odd group out.

== Adding Bullies

Thus far we have made certain assumptions about how to model a graph for this problem which may not be true in practice. Namely, that the only source of conflict between two students is a pre-existing partnership. One truth known all too well by Santa Claus is not all kids are nice. Some students may be "bullies" and have fewer than $n - 1$ students who are okay being paired with them. In the case of a true devil child with no friends, 0 iterations can be completed. Another upper bound on the number of rounds can be derived from the minimum degree of the graph, assuming the fewest edges being deleted in each iteration:

$ N_"friend_max" = delta(G) $

= Bigger Groups are NP-Complete ($k gt.eq 3$)

For groups of size $k gt.eq 3$, this problem is equivalent to finding repeated $k$-matchings with the added complexity of allowing larger degree matchings if needed. Consider the case where $k = 3$ and $n$ is a factor of $k$. Each round corresponds to finding a 3-dimensional matching, which is a known NP-complete problem. Any given answer is easy to verify, but computationally difficult to find. Finding a higher-degree matching (e.g., For the leftover students when $n$ is not a factor of $k$) is more difficult, and thus does not change the problem's overall complexity.

While the problem cannot be solved efficiently in the asymptotic sense, a backtracking algorithm is able to find the set of all "best" answers. An implementation and some test-cases using smaller values of $n$ is provided in #link("https://github.com/JBourds/graph-theory-final"). 
