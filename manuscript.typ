#let title = "Classroom Conundrum - Maximal Repeated Unique Group Assignments" 
#let due = datetime(year: 2025, month: 12, day: 9) 
#show link: set text(fill: blue) 
#show link: underline

#set page( paper: "us-letter",) 
#set par(justify: true) 
#set text(font: "Libertinus Serif", size: 11pt,)

#align(center)[ 
  #stack(dir: ttb, spacing: 1em, text(14pt, title, weight:
"bold"), text(12pt, "Jordan Bourdeau"), text( 12pt, due.display(),),) ]


= Problem Proposal & Background

It is common for instructors to incorporate group projects within their course
curriculum. These may use a single fixed group over the length of a class, or
assign new groups each time. The former of these approaches is boring from the
combinatorics perspective, so we ignore it. A natural question for any
instructor who loathes the idea of students experiencing repeated grouping to
ask themselves is whether their group assignments are possible without
duplicating groups over the course. We start with the simple and common case of
pairings, eventually working up to arbitrary group sizes. Specifically, we look
to answer the following:

- Given a class of _n_ students, how many unique groups with at least 2 members
  can we construct before being forced to repeat group members?
  - How does the parity of _n_ (odd or even) affect this?
  - How does this change given some initial set of conflicts between students?
  - How do these answers change when groups are of minimum size _k_, where _k_
    is some integer $gt.eq 2$?


= Partner Matchings (k = 2)

We represent the set of students, $J$, as vertices in the $G$. A student can
be placed into a group with any other student they share an edge with. After
each assignment, the edges between every student in each group will be
removed. A "conflicting" assignment occurs when two students are paired with
each other but do not contain an edge between them. How many times can this
process be repeated before a conflicting assignment is guaranteed to occur? 

== Simple Case

The simplest case to consider is when the initial graph is $K_n$ (given $n$
students) where $n$ is even. Because the initial graph is complete, every
edge can be treated the same. Since $n$ is a factor of the group size, 2,
there will be no uneven groups. Thus, a valid and optimal pairing is for
every vertex to be grouped with every other vertex over $n - 1$ rounds.

== Hopelessly Complex Case

When $n$ is odd, there must exist at least one grouping with size $gt.eq 3$. We 
assume an approach seeking to minimize the number of edges removed per
round, for which keeping equally sized groupings is advantageous. Thus, we would
decompose such a graph into $(n - 3) / 2$ groups with size 2 and one group of
size 3 in each round. Unfortunately, the problem already becomes incredibly 
difficult at this point. The parity of solutions results in an asymmetry which 
the graph property of interest is.

=== Similar Problem: Round-Robin Tournament

A commonly used strategy for similar matchmaking situations is the
round-robin tournament method. This technique adds "dummy" members to
round up to the nearest increment of the grouping size. A sample run for
even group sizes and an odd number of teams is given below (note D
represents the dummy, indicating a bye in the round):

#align(center)[ ```
1 2 3 4
5 6 7 D

1 5 2 3
6 7 D 4

1 6 5 2
7 D 4 3

1 7 6 5 D 4 3 2

1 D 7 6
4 3 2 5

1 4 D 7
3 2 5 6

1 3 4 D
2 5 6 7 ``` ]

Because instructors are unlikely to give students a bye on an assignment
due to parity of the class size, this won't fly. The lower bound on group
size means we must add remaining students into an existing groups, thereby
causing more edges to be deleted. 

== A Humble Upper Bound

When $k = 2$ and $n$ is odd, $(n + 3) / 2$ edges will be consumed per
assignment. From this we can extract a loose upper bound on the number of
possible iterations based on the maximum number of times this quantity of
edges could be removed from the complete graph:

$ N_"edge_max" = floor((|E(K_n)|) / ((n + 3) / 2)) = floor((2 |E(K_n)|) / (n + 3)) $

Even with this highly constrained variant of the problem, it is difficult
to establish a lower bound for a guaranteed number of iterations. There is no 
global way to partition sets like in
Turán graphs because each vertex has its own local view of which
vertices could be valid partners. Thus, even if a given vertex has enough
neighbors to create a group of size $k$, those neighbors may have some
conflict in a previous step preventing it. For instance, consider $K_n$ with
$n = 5$. The first iteration will select one $K_3$ and one $K_2$ whose
edges to remove. Afterward, there are no 3 vertices without conflicts to
create the odd group out.

== Adding Bullies

Thus far we have made certain assumptions about how to model a graph for
this problem which may not be true in practice. Namely, that the only
source of conflict between two students is a partnership from a previous round. 
One truth known all too well by Santa Claus is not everyone is nice. Some
students may be "bullies" and have fewer than $n - 1$ students who are
okay being paired with them. In the case of a true devil child with no
friends, 0 iterations can be completed. Another upper bound on the number
of rounds can be derived from the minimum degree of the graph. In this bound,
we assume the fewest edges being deleted from a vertex with this degree in each
round:


$ N_"friend_max" = delta(G) $

= Bigger Groups are NP-Complete ($k gt.eq 3$)

For groups of size $k gt.eq 3$, this problem is equivalent to finding
repeated $k$-dimensional matchings, with the added complexity of needing larger 
degree matchings when needed. Consider the case where $k = 3$, and $n$ is a factor
of $k$. Each round corresponds to finding a maximal 3-dimensional matching with 
no leftovers. This is a known NP-hard problem, suggesting the unique
group assignment problem is minimally NP-hard when $k gt.eq 3$. Note the
maximality condition is what makes this NP-hard, as simply checking for a valid
matching/unique group assignment would be possible in polynomial time.  

While the problem cannot be solved efficiently asymptotically, a
backtracking algorithm is able to find the set of all "best" answers. An
implementation and some test-cases using smaller values of $n$ is provided
in #link("https://github.com/JBourds/graph-theory-final"). 

== Upper Bounds when $k gt.eq 3$

Previously, we posed loose upper bounds for the case of pairings. We now
generalize these to arbitrary $k$ in graphs beyond $K_n$. 

The upper bound based on the edges deleted per iteration is more
challenging. Take some arbitrary $n$ and $k$ where $n gt.eq k and k gt 1$.
There will be $floor(n / k) * |E(K_k)|$ edges used by the subset of all
groupings containing only groups of size $k$. There are $l := n % k$
(where % is the modulus operator) students leftover which must be evenly
disbursed among the existing groups. In the case where the remaining
students can each be cleanly distributed into an existing group, the
number of edges consumed will increase by $l dot k$ per remaining student.
It could also be the case hat the leftover in a group exceeds the number
of existing groups, requiring multiple vertices be grouped into one
original $k$-sized group. Adding $j$ additional vertices to a $k$-group
causes $j k$ edge removals from every new vertex to each existing vertex,
as well as $(j (j - 1)) / 2$ edges being removed between each newly added
vertex. Thus, the total number of edges removed by merging a $K_j$ of
leftover vertices with the original $k$-group is $j k + (j (j - 1)) / 2$.
Expressing this as a summation is daunting, so it is provided over
multiple annotated steps below:

Calculate number of groups

$ n_"groups" := floor(n / k) $

Calculate number of leftover students 

$ l := n % k $

Calculate the number of edges used by the evenly split $k$-groupings

$ E_"groups" := n_"groups" * (|E(K_k)|) $

The number of remaining vertices cannot exceed a group's size, as
otherwise the vertices should form a new group rather than forming
leftover elements. When the number of leftover elements exceeds the number
of groups, groups may receive an uneven number of elements added to them.
Thus, each group will receive $floor(l / n_"groups")$ elements and $l %
n_"groups"$ groups will also get an additional elements. Let $j := floor(l
/ n_"groups")$.

Count the number of groups which receive $j + 1$ elements:

$ n_(j + 1) := l % n_"groups" $

Count the number of groups which receive $j$ elements:

$ n_j := n_"groups" - n_(j + 1) $

Thus, the total number of additional edges being removed by assigning the
leftover vertices each iteration is:

$ E_"leftovers" := n_(j + 1) ((j + 1) k + ((j + 1) j) / 2) + n_j (j k + (j
(j
- 1)) / 2) $ 

The total number of edges between both the original $k$-sized groupings
and leftover vertices is:

$ E_"total" := E_"groups" + E_"leftovers" $

The upper bound based on the minimum edge degree is easy to come up with
since the only thing which must change is the denominator. Previously, there
was an implicit 1 in the denominator as that is the number of edges in
$K_2$. Now, we explicitly include the number of edges consumed each round in
the denominator. This includes the $k$-sized grouping and any leftover vertices
which must be added. Because this is for an upper bound, we stay conservative
and assume the fewest number of additional vertices get added. Thus, the
denominator is the number of edges in a clique of size $k + j$, rather than
$k + j + 1$.

$ N_"friend_max" = delta(G) / (|E(K_(k+j))|)  $

= Conclusion

Maximal repeated unique group assignments is a problem which very quickly
becomes incredibly difficult due to the dynamic nature of forbidden assignments 
for each vertex. In this paper we demonstrated it is an NP-hard problem and
proposed upper bounds for how many unique group assignments are possible based
on the edges consumed per iteration and minimum degree. Included as source code
is an implementation of a backtracking algorithm for coming up with all possible
"best" solutions for this problem.
