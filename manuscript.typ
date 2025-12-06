#let title = "Classroom Conundrum - Maximal Repeated Unique Group Assignments"
#let due = datetime(year: 2025, month: 12, day: 9)
#show link: set text(fill: blue)
#show link: underline

#set page(paper: "us-letter")
#set par(justify: true)
#set text(font: "Libertinus Serif", size: 11pt)

#align(center)[
  #stack(
    dir: ttb,
    spacing: 1em,
    text(14pt, title, weight: "bold"),
    text(12pt, "Jordan Bourdeau"),
    text(12pt, due.display()),
  ) ]


= Problem Proposal & Background

It is common for instructors to incorporate group projects within their course
curriculum. These may use a single fixed group over the length of a class, or
assign new groups each time. The former of these approaches is boring from the
combinatorics perspective, so we ignore it. A natural question for
instructors who loathe repeated group assignments is whether it is actually
possible to have enough unique groups in a semester. We start with the simple
and common case of pairings, eventually working up to arbitrary group sizes.
Specifically, we look to answer the following:

- Given a class of _n_ students, how many unique pairings without repeating any student group?
  - How does the parity of _n_ (odd or even) affect this?
  - How does this change given some initial set of conflicts between students?
- How do these answers change when groups are of minimum size _k_, where _k_
  is some integer $gt 2$?

= Partner Matchings (k = 2)

We represent the set of students, $J$, as vertices in the graph $G$. Note that
we use the terms "student" and "vertex" interchangably here on out. A student can
be placed into a group with any other student they share an edge with. After
each assignment, the edges between every student in each group will be
removed. A "conflicting" assignment occurs when two students are paired together
but do not share an edge between them in the initial graph. In other words,
these students have never been grouped together before.

=== Similar Problem: Round-Robin Tournament

A commonly used strategy for similar matchmaking situations is the
round-robin tournament method. When there are an even number of teams, each team
will play each other a single time. When there are an odd number of teams, this
technique adds "dummy" members to round up to the nearest increment of the group
size. A sample run for even group sizes and an odd number of teams is given
below for $k = 2$. Note that D is the "dummy", and gives the paired team a bye
for that round.

#align(center)[ ```
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
2 5 6 7 ``` ]

Because instructors are unlikely to give students a bye on an assignment
due to parity of the class size, this won't fly. As formulated, this problem
enforces a lower bound on group sizes so we can never remove fewer edges than
each student being matched into a pairing.


== Simple Case

The simplest case to consider is when there are an even number of students.
We can start with the complete graph $K_n$. Since the graph is complete, every
edge represents a potential pairing between students, and all edges are equally
valid for forming groups. Because $n$ is a factor of the group size, 2, there
will be no uneven groups due to parity. Thus, an optimal conflict-free set of
assignments is to pair every student with every other student over $n - 1$
rounds. This is exactly the Round-Robin tournament problem with an even number
of teams.

== Harder Case

When $n$ is odd, there must exist at least one grouping with size $gt.eq 3$.
Because the number of edges within a clique increase quadratically with its
size, keeping groups small is a good strategy for minimizing edge deletions
each round. The best strategy to do this is to decompose a graph into
$(n - 3) / 2$ groups with size 2 and one group of size 3 in each round.

== A Humble Upper Bound

When $k = 2$ and $n$ is odd, $(n + 3) / 2$ edges will be consumed per
assignment. We come to this number via simple algebra on the number of two
and three person groups and the edges consumed by each group size:

$
  1 dot ((n - 3) / 2) + 3 dot 1 \
  = ((n - 3) / 2) + 6 / 2 \
  = (n + 3) / 2
$

From this we can extract a loose upper bound on the number of
possible iterations based on the maximum number of times this quantity of
edges could be removed from the complete graph:

$
  N_"edge_max" = floor((|E(K_n)|) / ((n + 3) / 2)) = floor((2 |E(K_n)|) / (n + 3))
$

Even with this highly constrained variant of the problem, it is unclear how
to establish a lower bound for a guaranteed number of iterations. There is no
global way to partition sets like in the Turán graph, and each vertex has a
local view of what valid partners are based on prior assignments. Thus, even if
a given vertex has enough neighbors to create a group of size $k$, those
neighbors may have a prior conflict preventing the match.

For example, consider $K_n$ with
$n = 5$. There are 10 edges in this graph, and each grouping will consume 4 of
those edges. However, only one assignment can be made due to there being no
$K_3$ subgraph after the first grouping.

== Adding Bullies

Up to this point we have made certain assumptions about how to model a graph for
this problem which may not be true in practice. Namely, that the only
source of conflict between two students is a partnership from a previous round.
One truth known all too well by Santa Claus is not everyone is nice. Some
students may be "bullies" and have fewer than $n - 1$ students
willing to be paired with them. A true devil child may have no edges in the
graph, meaning any assignment at all is impossible. This greatly complicates the
problem since there are no assumptions we can make about graph structure.
However, introducing non-uniformity into the initial graph does provide another
source for an upper bound based on the minimum degree of a graph. For this
bound, we assume the minimum edges being deleted from a vertex with minimum
degree. When $k = 2$, each student loses a minimum of one edge per round.
Therefore, we could at most have a number of assignments equal to the minimum
degree.

$ N_"friend_max" = delta(G) $

= Bigger Groups are NP-Hard ($k gt.eq 3$)

For groups of size $k gt.eq 3$, this problem is equivalent to finding
repeated $k$-dimensional perfect matchings which also require larger degree
matchings for odd parities. When $k = 3$ and $n$ is a multiple
of $k$, each round corresponds to finding a perfect 3-dimensional matching with
no leftovers. This is a known NP-hard problem, and thus each round in the
unique group assignment problem is equivalent to an NP-hard problem when
$k gt.eq 3$.
When $n$ is not a multiple of $k$, verifying a candidate round is
still easily done in polynomial time by checking that each matching has at least
$k$ students and every student in each group contains an edge to each other.
This can be checked in $O(n k)$ time. Higher dimensional requirements do not
change the complexity of the problem, even though the computational demands
would greatly increase.

= Actually, it's NP-Super Duper Hard

When $k gt.eq 3$, each individual round of group formation already requires
solving an NP-hard problem. For $k = 3$, this is exactly the well-known
Perfect 3-Dimensional Matching problem, which is NP-hard. The same reasoning
extends to all fixed $k gt.eq 3$.

This observation immediately implies the repeated unique group assignment
problem is NP-hard, because even deciding whether a single conflict-free
assignment exists is NP-hard. More formally:

*Theorem*

For every integer $k gt.eq 3$, performing repeated unique group assigment (RUGA)
is NP-hard.

*Proof.*

We reduce the NP-hard Perfect 3-Dimensional Matching (3DM) problem to a
single round of RUGA with group size $k = 3$ where $n$ is a factor of $k$.
Given an instance of 3DM consisting of disjoint sets $X, Y, Z$ and the set of
possible triples $T subset.eq X crossmark Y crossmark Z$, construct a graph
where each triple $(x, y, z) in T$ forms a $K_3$ triangle.

A single round of RUGA with $k = 3$ can form a set of group assignments covering
all vertices iff the original 3DM instance has a perfect matching. This
reduction preserves solutions and is computable in polynomial time. Thus, RUGA
is NP-hard. $qed$

== It might be even harder than that

The NP-hardness result from earlier applies to a single round. The actual RUGA
problem asks for something much harder: an optimal sequence of such rounds to
maximize the number of non-conflicting group assignments. Because even
constructing a single round is NP-hard, constructing a sequence of rounds
is at least as hard. This problem exhibits the characteristic alternation of
existential and universal constraints seen in problems from the second level of
the polynomial hierarchy. Problems in the  $Sigma_2^P$ complexity class are
typically of the form:

$ exists x forall y: P(x, y) $

Another way to characterize this complexity class is as problems which could be
solved in polynomial time if we had access to an NP oracle function. For RUGA,
if we had access to an NP oracle for solving individual round assignments the
problem could be solved in polynomial time. Mapping RUGA to the nested predicate
from earlier, we get:

$
  exists ("sequence of groupings") \
  forall ("constraints imposed by edge removals in earlier rounds"): \
  "NP operation to get an assignment"
$

Thus the overall problem of maximizing the number of rounds in RUGA is NP-hard,
even when restricted to deciding whether at least one round exists, and
likely belongs to a higher complexity class such as $Sigma_2^P$.

While the problem cannot be solved efficiently asymptotically, a
backtracking algorithm is able to find the set of all "best" answers. An
implementation and some test-cases using smaller values of $n$ is provided
in #link("https://github.com/JBourds/graph-theory-final"). Note the
computational demands of this problem balloon rapidly and values of $n gt.eq 10$
take a long time to run.

== Upper Bounds when $k gt.eq 3$

Previously, we posed loose upper bounds for the case of pairings. We now
generalize these to arbitrary $k$ in graphs beyond $K_n$. We start by looking at
the number of edges deleted per iteration.

Take some arbitrary $n$ and $k$ where $n gt.eq k and k gt 1$.
There will be $floor(n / k) * |E(K_k)|$ edges used by the subset of all
groupings containing only groups of size $k$. There are $l := n " " % " " k$
(% is the modulus operator) students leftover which must be
disbursed among the existing groups. Because the number of edges deleted by a
grouping grows quadratically as the number of students increases, the strategy
to minimize edge deletions is to evenly distribute remaining students across
groups. Let $n_G$ be the number of groups. If $l lt.eq n_G$, the
number of edges deleted will increase by $l dot k$.
If $l gt n_G$, multiple students must be redistributed into some existing group.
Adding $j$ additional vertices to a $k$-group
causes $j k$ edge removals from every new vertex to each existing vertex,
as well as $(j (j - 1)) / 2$ edges being removed between each newly added
vertex. Thus, the total number of edges removed by merging a $K_j$ of
leftover vertices with the original $k$-group is $j k + (j (j - 1)) / 2$.
Expressing this as a summation is daunting, so it is provided over
multiple annotated steps below:

Calculate number of groups

$ n_G := floor(n / k) $

Calculate number of leftover students

$ l := n " " % " " k $

Calculate the number of edges used by the evenly split $k$-groupings

$ E_G := n_G * (|E(K_k)|) $

The number of remaining vertices cannot exceed a group's size, as
otherwise the vertices should form a new group rather than forming
leftover elements. When the number of leftover elements exceeds the number
of groups, groups may receive an uneven number of elements added to them.
Thus, each group will receive $floor(l / n_G)$ elements and $l " " %
" " n_G$ groups will receive one additional student. Let $j := floor(
  l
  / n_G
)$.

Count the number of groups which receive $j + 1$ elements:

$ n_(j + 1) := l " " % " " n_G $

Count the number of groups which receive $j$ elements:

$ n_j := n_G - n_(j + 1) $

Thus, the total number of additional edges being removed by assigning the
leftover vertices each iteration is:

$
  E_L := n_(j + 1) ((j + 1) k + ((j + 1) j) / 2) + n_j (j k + (j
    (j
      - 1)) / 2)
$

The total number of edges between both the original $k$-sized groupings
and leftover vertices is:

$ E_T := E_G + E_L $

Thus, a loose upper bound for how many times this procedure could possible be
applied to a graph $G$ is given by:

$ floor((|E(G)|) / E_T) $

The upper bound based on the minimum edge degree is easy to come up with
since the only thing which must change is the denominator. Previously, there
was an implicit 1 in the denominator as that is the number of edges in
$K_2$. Now, we explicitly include the number of edges consumed each round in
the denominator. This includes the $k$-sized grouping and any leftover vertices
which must be added. Because this is for an upper bound, we stay conservative
and assume the fewest number of additional vertices get added. Thus, the
denominator is the number of edges in a clique of size $k + j$, rather than
$k + j + 1$.

$ N_"friend_max" = delta(G) / (|E(K_(k+j))|) $

= Conclusion

Maximal repeated unique group assignments is a problem which very quickly
becomes incredibly difficult due to the dynamic nature of forbidden assignments
for each vertex. We demonstrated it is an NP-hard problem, conjectured it may
also exist in a higher complexity class such as $Sigma_2^P$, and
established upper bounds for how many unique group assignments are possible based
on the edges consumed per iteration and minimum degree. Included as source code
is an implementation of a backtracking algorithm for coming up with all possible
"best" solutions for this problem.

#bibliography("sources.bib", full: true)

