# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.134 | 7.44M | 0.134 | 7.45M | 0.195 | 1.45× | 1.45× |
| 10,000 | 1.349 | 7.41M | 1.315 | 7.60M | 1.720 | 1.27× | 1.31× |
| 100,000 | 13.785 | 7.25M | 13.158 | 7.60M | 17.005 | 1.23× | 1.29× |
| 1,000,000 | 132.901 | 7.52M | 133.720 | 7.48M | 168.488 | 1.27× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.097 | 1.06× |
| 1 | 5 | 0.297 | 0.390 | 1.31× |
| 1 | 10 | 0.512 | 0.847 | 1.66× |
| 10 | 1 | 0.048 | 0.086 | 1.79× |
| 10 | 5 | 0.242 | 0.423 | 1.75× |
| 10 | 10 | 0.483 | 0.909 | 1.88× |
| 100 | 1 | 0.071 | 0.103 | 1.45× |
| 100 | 5 | 0.241 | 0.563 | 2.33× |
| 100 | 10 | 0.534 | 1.142 | 2.14× |
| 1,000 | 1 | 0.221 | 0.306 | 1.38× |
| 1,000 | 5 | 0.395 | 1.356 | 3.44× |
| 1,000 | 10 | 0.657 | 2.811 | 4.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
