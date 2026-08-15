# Liquidity benchmark (`causal liquidity pools` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.79M | 0.033 | 30.54M | 4.632 | 128.73× | 141.46× |
| 10,000 | 0.387 | 25.83M | 0.382 | 26.21M | 68.067 | 175.83× | 178.41× |
| 100,000 | 4.380 | 22.83M | 4.183 | 23.91M | 1135.691 | 259.28× | 271.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.183 | 1.44× |
| 1 | 5 | 0.378 | 0.580 | 1.53× |
| 1 | 10 | 0.396 | 1.119 | 2.82× |
| 10 | 1 | 0.053 | 0.124 | 2.33× |
| 10 | 5 | 0.191 | 0.593 | 3.10× |
| 10 | 10 | 0.410 | 1.197 | 2.92× |
| 100 | 1 | 0.048 | 0.203 | 4.21× |
| 100 | 5 | 0.202 | 1.009 | 4.99× |
| 100 | 10 | 0.456 | 2.010 | 4.41× |
| 1,000 | 1 | 0.092 | 4.932 | 53.77× |
| 1,000 | 5 | 0.239 | 24.936 | 104.49× |
| 1,000 | 10 | 0.591 | 54.224 | 91.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
