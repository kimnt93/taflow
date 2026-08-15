# CypherPattern benchmark (`Cypher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.31M | 0.008 | 131.00M | 0.233 | 24.12× | 30.59× |
| 10,000 | 0.091 | 109.43M | 0.085 | 117.79M | 1.417 | 15.51× | 16.69× |
| 100,000 | 0.852 | 117.31M | 0.842 | 118.83M | 13.124 | 15.40× | 15.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.221 | 2.38× |
| 1 | 5 | 0.267 | 0.840 | 3.14× |
| 1 | 10 | 0.411 | 1.728 | 4.20× |
| 10 | 1 | 0.042 | 0.165 | 3.92× |
| 10 | 5 | 0.203 | 1.159 | 5.72× |
| 10 | 10 | 0.419 | 1.750 | 4.18× |
| 100 | 1 | 0.050 | 0.176 | 3.52× |
| 100 | 5 | 0.196 | 1.148 | 5.84× |
| 100 | 10 | 0.409 | 1.830 | 4.48× |
| 1,000 | 1 | 0.064 | 0.306 | 4.76× |
| 1,000 | 5 | 0.204 | 1.785 | 8.75× |
| 1,000 | 10 | 0.441 | 3.066 | 6.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
