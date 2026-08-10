# BarsSince benchmark (`bars since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 187.41M | 0.005 | 218.28M | 0.117 | 21.95× | 25.57× |
| 10,000 | 0.029 | 343.66M | 0.027 | 374.69M | 1.076 | 36.99× | 40.33× |
| 100,000 | 0.252 | 396.28M | 0.227 | 440.06M | 10.532 | 41.74× | 46.35× |
| 1,000,000 | 2.681 | 373.05M | 2.220 | 450.39M | 119.176 | 44.46× | 53.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.097 | 1.00× |
| 1 | 5 | 0.320 | 0.291 | 0.91× |
| 1 | 10 | 0.468 | 0.608 | 1.30× |
| 10 | 1 | 0.049 | 0.057 | 1.18× |
| 10 | 5 | 0.214 | 0.286 | 1.34× |
| 10 | 10 | 0.437 | 0.629 | 1.44× |
| 100 | 1 | 0.051 | 0.071 | 1.38× |
| 100 | 5 | 0.215 | 0.347 | 1.62× |
| 100 | 10 | 0.478 | 0.703 | 1.47× |
| 1,000 | 1 | 0.049 | 0.166 | 3.41× |
| 1,000 | 5 | 0.209 | 0.836 | 4.01× |
| 1,000 | 10 | 0.498 | 1.694 | 3.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
