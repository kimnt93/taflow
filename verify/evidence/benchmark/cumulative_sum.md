# CumulativeSum benchmark (`numpy.cumsum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 397.85M | 0.002 | 610.51M | 0.016 | 6.43× | 9.87× |
| 10,000 | 0.012 | 802.53M | 0.010 | 980.16M | 0.036 | 2.85× | 3.49× |
| 100,000 | 0.126 | 791.81M | 0.092 | 1.08G | 0.220 | 1.74× | 2.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.077 | 1.20× |
| 1 | 5 | 0.351 | 0.349 | 0.99× |
| 1 | 10 | 0.394 | 0.646 | 1.64× |
| 10 | 1 | 0.042 | 0.071 | 1.69× |
| 10 | 5 | 0.183 | 0.298 | 1.62× |
| 10 | 10 | 0.381 | 0.616 | 1.62× |
| 100 | 1 | 0.043 | 0.066 | 1.51× |
| 100 | 5 | 0.169 | 0.291 | 1.73× |
| 100 | 10 | 0.406 | 0.644 | 1.59× |
| 1,000 | 1 | 0.046 | 0.063 | 1.38× |
| 1,000 | 5 | 0.186 | 0.359 | 1.93× |
| 1,000 | 10 | 0.376 | 0.700 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
