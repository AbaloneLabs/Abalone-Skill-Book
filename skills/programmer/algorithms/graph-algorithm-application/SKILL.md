---
name: graph_algorithm_application.md
description: Use when the agent is modeling a problem as a graph and selecting or implementing graph algorithms — shortest path (Dijkstra, A*, Bellman-Ford), traversal (BFS, DFS), connectivity (connected components, SCC), centrality (PageRank, betweenness), community detection, minimum spanning tree, network flow (max-flow/min-cut), topological sort; choosing graph representations (adjacency list, matrix); or reasoning about graph properties (directed/undirected, weighted, cyclic, planar). Covers problem-to-graph modeling, algorithm selection by graph properties, representation tradeoffs, the cost of the wrong algorithm (e.g., Bellman-Ford vs Dijkstra), and the distinction between graph problems that have efficient algorithms and those that are NP-hard.
---

# Graph Algorithm Application

Many real-world problems are graph problems in disguise — network routing, dependency resolution, social network analysis, recommendation, scheduling, circuit design, molecular analysis — and recognizing the graph structure is the first step to solving them efficiently. The power of graph algorithms is that a vast body of proven, efficient algorithms exists for standard graph problems (shortest path, connectivity, traversal, flow), so a problem modeled as a graph inherits decades of optimized solutions. The hazard is that the modeling and the algorithm selection are consequential: the same problem modeled as a directed vs undirected graph, or with weights vs unweighted, changes which algorithm applies; choosing the wrong algorithm (Bellman-Ford where Dijkstra suffices, or a general algorithm where a specialized one exists) can turn a linear-time solution into a quadratic or cubic one; and some graph problems (traveling salesman, graph coloring, maximum clique) are NP-hard, with no efficient exact solution, so recognizing them prevents a futile search for a polynomial algorithm.

Agents tend to reach for a familiar graph algorithm without analyzing the graph's properties (weighted? directed? cyclic? sparse or dense?), to miss that a problem is a graph problem at all (modeling it as nested loops instead), or to attempt exact solutions to NP-hard graph problems. The judgment problem is recognizing graph structure in problems, matching the algorithm to the graph's properties and the problem's constraints, choosing the right representation, and distinguishing tractable from intractable graph problems. This skill covers the discipline of graph algorithm application: problem modeling, algorithm selection, representation, and the tractability boundary.

## Core Rules

### Model The Problem As A Graph Correctly

The first step is recognizing that a problem has graph structure and modeling it faithfully: identifying the vertices, edges, and properties (direction, weight, multiplicity) that capture the problem.

- **Identify the vertices (nodes) and edges (relationships).** In a routing problem, vertices are locations and edges are connections; in a dependency problem, vertices are tasks and edges are dependencies; in a social network, vertices are people and edges are relationships. The modeling determines what the graph captures.
- **Determine if edges are directed or undirected.** A dependency (A requires B) is directed; a friendship (A and B are friends) is undirected. Directionality determines the applicable algorithms (topological sort requires a directed acyclic graph; connected components apply to undirected).
- **Determine if edges are weighted and what the weight means.** A routing graph's edge weight is distance or time; a cost graph's weight is cost. Weighted graphs need weighted algorithms (Dijkstra, Bellman-Ford); unweighted use BFS. The weight's meaning (cost to minimize, capacity to maximize, strength of connection) determines the objective.
- **Determine if the graph is cyclic.** A dependency graph should be acyclic (a cycle is a circular dependency, an error); a social network is cyclic (A-B-C-A). Cyclicity affects algorithms (DAGs allow topological sort; cyclic graphs need SCC for strong structure).
- **Capture all relevant properties.** Multiple edges between the same vertices (multigraph), self-loops, edge labels, vertex attributes — capture what the problem needs, and no more (over-modeling adds complexity).

### Choose The Algorithm Based On Graph Properties And Constraints

Graph algorithms have preconditions (the graph properties they require) and complexities (their time/space cost). Match the algorithm to the graph and the constraints.

- **Shortest path: match the algorithm to the graph's weights and structure.** Unweighted: BFS (O(V+E)). Non-negative weights: Dijkstra (O((V+E) log V) with a heap). Negative weights (no negative cycle): Bellman-Ford (O(VE)) — slower but handles negatives. Negative cycles: detectable by Bellman-Ford; shortest path is undefined. All-pairs: Floyd-Warshall (O(V^3)) for dense, or repeated Dijkstra for sparse.
- **A* for shortest path with a heuristic.** When a heuristic estimates the distance to the target (geographic distance, Manhattan distance), A* explores toward the target, often far faster than Dijkstra. The heuristic must be admissible (never overestimates) for optimality.
- **Connectivity: connected components (undirected), SCC (directed).** Connected components (via BFS/DFS or union-find) find the disconnected subgraphs of an undirected graph. Strongly connected components (Tarjan's or Kosaraju's) find the maximal strongly connected subgraphs of a directed graph — used for dependency analysis, cycle detection.
- **Topological sort for DAGs.** A linear ordering of a DAG's vertices where each edge points forward — used for dependency resolution, scheduling, build ordering. Requires a DAG; a cycle means no valid ordering (a circular dependency).
- **Minimum spanning tree (Kruskal's, Prim's).** The minimum-weight tree connecting all vertices of an undirected weighted graph — used for network design, clustering. Kruskal's (edge-based, with union-find) suits sparse graphs; Prim's (vertex-based) suits dense.
- **Network flow (max-flow, min-cut).** The maximum flow from a source to a sink through a capacitated graph, equivalent to the minimum cut separating them — used for bipartite matching, network reliability, image segmentation. Ford-Fickerson/Edmonds-Karp or Dinic's algorithm.

### Choose The Right Representation For The Graph And Operations

The graph representation (adjacency list, adjacency matrix, edge list) affects memory and the speed of operations. Match it to the graph's density and the operations needed.

- **Adjacency list: each vertex stores its neighbors.** Memory O(V+E), efficient for sparse graphs (most real-world graphs are sparse). Checking an edge is O(degree); iterating neighbors is O(degree). The default for most graph algorithms.
- **Adjacency matrix: a V×V matrix of edge presence/weight.** Memory O(V^2), efficient for dense graphs. Checking an edge is O(1); iterating neighbors is O(V). Used for dense graphs or algorithms needing fast edge lookup (Floyd-Warshall).
- **Edge list: a list of edges.** Memory O(E), simple but slow for most operations (finding a vertex's neighbors requires scanning all edges). Used for algorithms that process edges (Kruskal's sorts the edge list).
- **Match the representation to the graph and operations.** Sparse graph with neighbor iteration: adjacency list. Dense graph with edge lookup: adjacency matrix. Edge-centric algorithm: edge list. The wrong representation can turn an efficient algorithm into an inefficient one.

### Recognize NP-Hard Graph Problems And Use Approximation Or Heuristics

Some graph problems are NP-hard — no known polynomial-time exact algorithm exists. Recognizing them prevents a futile search for an efficient exact solution.

- **Traveling salesman (TSP), graph coloring, maximum clique, minimum vertex cover, longest path: NP-hard.** These problems have no efficient exact solution for general graphs; attempting exact solutions on large graphs is infeasible. Recognize them; do not search for a polynomial exact algorithm.
- **Use approximation algorithms for NP-hard problems.** Many NP-hard graph problems have approximation algorithms with proven bounds (a 2-approximation for vertex cover, a O(log n)-approximation for set cover). Use these for a guaranteed-near-optimal solution in polynomial time.
- **Use heuristics for problems without good approximations.** TSP has heuristics (nearest neighbor, 2-opt) that work well in practice without guarantees. Use them where an approximation bound is unavailable and empirical performance suffices.
- **Exploit special graph structure.** Some NP-hard problems are tractable on restricted graph classes (TSP on a tree is trivial; coloring on a bipartite graph is 2-colorable in polynomial time). If the graph has special structure, a tractable algorithm may exist.

### Verify The Algorithm Handles The Graph's Edge Cases

Graph algorithms have edge cases (disconnected graphs, self-loops, parallel edges, zero-weight or negative edges) that some implementations mishandle.

- **Disconnected graphs.** A shortest-path algorithm run from one vertex reaches only its component; to reach all, run from each component or use an algorithm that handles disconnection. Connected components must be enumerated, not assumed.
- **Self-loops and parallel edges.** A self-loop (an edge from a vertex to itself) or parallel edges (multiple edges between the same vertices) may break an algorithm's assumptions or inflate results. Handle or exclude them as the problem requires.
- **Zero-weight or negative edges.** Dijkstra assumes non-negative weights; a negative edge breaks it (it may not find the shortest path). Bellman-Ford handles negatives; use it where negatives are possible.
- **Empty or single-vertex graphs.** Algorithms should handle trivial graphs (no vertices, one vertex) without error. Test these edge cases.

## Common Traps

### Wrong Algorithm For The Graph's Properties

Dijkstra on a graph with negative edges (breaks), BFS on a weighted graph (ignores weights), Bellman-Ford where Dijkstra suffices (unnecessarily slow). Match the algorithm to the properties.

### Missing That A Problem Is A Graph Problem

Modeling a graph problem (dependency resolution, network routing) as nested loops or ad-hoc recursion, missing the efficient graph algorithm. Recognize graph structure.

### Wrong Representation For The Graph's Density

An adjacency matrix for a sparse graph (O(V^2) memory wasted), or an adjacency list for a dense graph needing fast edge lookup. Match the representation to density and operations.

### Attempting Exact Solutions To NP-Hard Problems

Searching for a polynomial exact algorithm for TSP, graph coloring, or maximum clique, which are NP-hard. Recognize NP-hardness; use approximation or heuristics.

### Ignoring Edge Cases (Disconnection, Negatives, Self-Loops)

An algorithm that mishandles disconnected graphs, negative edges, or self-loops, producing incorrect results or infinite loops. Test edge cases.

### Inappropriate Heuristic For A*

A heuristic that overestimates (not admissible), causing A* to find a non-optimal path, or a heuristic that is too weak, causing A* to behave like Dijkstra. Ensure admissibility; tune the heuristic.

### Topological Sort On A Cyclic Graph

Attempting topological sort on a graph with a cycle, which has no valid ordering (the cycle is a circular dependency). Detect cycles; handle them as errors.

### Over-Modeling The Graph

Capturing properties the problem does not need (edge labels, vertex attributes, directions that do not matter), adding complexity without benefit. Model what the problem needs.

## Self-Check

- [ ] The problem is correctly modeled as a graph — vertices, edges, direction (directed/undirected), weights (and their meaning), cyclicity, and other relevant properties are identified and captured, without over-modeling properties the problem does not need.
- [ ] The algorithm is chosen based on the graph's properties and constraints — shortest path (BFS unweighted, Dijkstra non-negative, Bellman-Ford negative, A* with heuristic), connectivity (components/SCC), topological sort (DAGs), MST (Kruskal/Prim), flow (max-flow) — matching preconditions and complexities.
- [ ] The graph representation (adjacency list, matrix, edge list) matches the graph's density and the operations needed, so memory and operation costs are appropriate rather than inflated by a mismatch.
- [ ] NP-hard graph problems (TSP, coloring, maximum clique, longest path) are recognized, and exact solutions are not attempted on large graphs — approximation algorithms (with proven bounds) or heuristics are used instead, with special graph structure exploited where it makes the problem tractable.
- [ ] The algorithm handles edge cases: disconnected graphs (enumerate components, do not assume connectivity), self-loops and parallel edges, zero-weight or negative edges (use the correct algorithm), and empty or single-vertex graphs.
- [ ] For A*, the heuristic is admissible (never overestimates) to guarantee optimality, and it is strong enough to guide the search efficiently without being so weak that A* degenerates to Dijkstra.
- [ ] The algorithm's complexity (time and space) is understood and acceptable for the problem's scale — a cubic algorithm (Floyd-Warshall) on a large graph is infeasible; a linear or near-linear algorithm (BFS, Dijkstra) is preferred where it suffices.
- [ ] The choice between graph algorithms has been validated against the problem's requirements (does it need the shortest path, all paths, connectivity, ordering?) and the graph's reality (is it sparse or dense, weighted or unweighted, static or changing), rather than defaulting to a familiar algorithm regardless of fit.
