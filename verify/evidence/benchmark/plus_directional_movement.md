# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.45M | 0.006 | 168.64M | 0.037 | 5.06× | 6.30× |
| 10,000 | 0.056 | 178.43M | 0.051 | 195.15M | 0.082 | 1.46× | 1.59× |
| 100,000 | 0.521 | 191.76M | 0.511 | 195.78M | 0.529 | 1.01× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.188 | 0.109 | 0.58× |
| 1 | 5 | 0.279 | 0.518 | 1.85× |
| 1 | 10 | 0.392 | 0.919 | 2.34× |
| 10 | 1 | 0.041 | 0.093 | 2.29× |
| 10 | 5 | 0.175 | 0.456 | 2.61× |
| 10 | 10 | 0.399 | 0.943 | 2.36× |
| 100 | 1 | 0.040 | 0.088 | 2.21× |
| 100 | 5 | 0.190 | 0.435 | 2.28× |
| 100 | 10 | 0.391 | 0.941 | 2.41× |
| 1,000 | 1 | 0.050 | 0.102 | 2.02× |
| 1,000 | 5 | 0.228 | 0.488 | 2.14× |
| 1,000 | 10 | 0.442 | 0.975 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
