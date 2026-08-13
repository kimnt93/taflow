# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.144 | 6.93M | 0.140 | 7.14M | 0.042 | 0.29× | 0.30× |
| 10,000 | 1.335 | 7.49M | 1.372 | 7.29M | 0.143 | 0.11× | 0.10× |
| 100,000 | 14.334 | 6.98M | 13.814 | 7.24M | 1.168 | 0.08× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.166 | 1.12× |
| 1 | 5 | 0.336 | 0.481 | 1.43× |
| 1 | 10 | 0.632 | 0.974 | 1.54× |
| 10 | 1 | 0.070 | 0.108 | 1.54× |
| 10 | 5 | 0.310 | 0.460 | 1.49× |
| 10 | 10 | 0.631 | 0.969 | 1.53× |
| 100 | 1 | 0.079 | 0.104 | 1.33× |
| 100 | 5 | 0.310 | 0.457 | 1.47× |
| 100 | 10 | 0.627 | 0.981 | 1.57× |
| 1,000 | 1 | 0.199 | 0.109 | 0.55× |
| 1,000 | 5 | 0.438 | 0.528 | 1.20× |
| 1,000 | 10 | 0.773 | 1.079 | 1.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
