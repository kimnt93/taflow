# MathAtanh benchmark (`numpy.arctanh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.96M | 0.031 | 32.56M | 0.025 | 0.70× | 0.81× |
| 10,000 | 0.253 | 39.53M | 0.241 | 41.41M | 0.144 | 0.57× | 0.60× |
| 100,000 | 2.407 | 41.54M | 2.358 | 42.41M | 1.359 | 0.56× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.079 | 0.65× |
| 1 | 5 | 0.421 | 0.295 | 0.70× |
| 1 | 10 | 0.547 | 0.562 | 1.03× |
| 10 | 1 | 0.065 | 0.054 | 0.84× |
| 10 | 5 | 0.282 | 0.281 | 1.00× |
| 10 | 10 | 0.575 | 0.559 | 0.97× |
| 100 | 1 | 0.064 | 0.062 | 0.97× |
| 100 | 5 | 0.279 | 0.265 | 0.95× |
| 100 | 10 | 0.566 | 0.581 | 1.03× |
| 1,000 | 1 | 0.089 | 0.077 | 0.86× |
| 1,000 | 5 | 0.280 | 0.309 | 1.10× |
| 1,000 | 10 | 0.576 | 0.748 | 1.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
