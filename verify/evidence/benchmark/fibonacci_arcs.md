# FibonacciArcs benchmark (`FibArcs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.102 | 9.83M | 0.086 | 11.65M | 0.524 | 5.15× | 6.11× |
| 10,000 | 0.848 | 11.79M | 0.784 | 12.75M | 3.879 | 4.57× | 4.95× |
| 100,000 | 8.324 | 12.01M | 7.783 | 12.85M | 41.496 | 4.99× | 5.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.214 | 0.207 | 0.97× |
| 1 | 5 | 0.398 | 0.900 | 2.26× |
| 1 | 10 | 0.653 | 1.866 | 2.86× |
| 10 | 1 | 0.071 | 0.174 | 2.43× |
| 10 | 5 | 0.299 | 0.842 | 2.82× |
| 10 | 10 | 0.599 | 1.894 | 3.16× |
| 100 | 1 | 0.078 | 0.213 | 2.71× |
| 100 | 5 | 0.309 | 1.035 | 3.35× |
| 100 | 10 | 0.655 | 2.264 | 3.46× |
| 1,000 | 1 | 0.161 | 0.770 | 4.79× |
| 1,000 | 5 | 0.310 | 3.143 | 10.15× |
| 1,000 | 10 | 0.708 | 6.418 | 9.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
