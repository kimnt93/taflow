# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.85M | 0.011 | 94.10M | 0.038 | 2.75× | 3.60× |
| 10,000 | 0.116 | 86.36M | 0.122 | 82.20M | 0.134 | 1.16× | 1.10× |
| 100,000 | 1.175 | 85.08M | 1.088 | 91.87M | 1.047 | 0.89× | 0.96× |
| 1,000,000 | 11.462 | 87.24M | 11.399 | 87.73M | 9.870 | 0.86× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.111 | 0.76× |
| 1 | 5 | 0.307 | 0.488 | 1.59× |
| 1 | 10 | 0.640 | 0.900 | 1.41× |
| 10 | 1 | 0.052 | 0.091 | 1.75× |
| 10 | 5 | 0.236 | 0.405 | 1.72× |
| 10 | 10 | 0.517 | 1.104 | 2.13× |
| 100 | 1 | 0.063 | 0.098 | 1.55× |
| 100 | 5 | 0.263 | 0.449 | 1.71× |
| 100 | 10 | 0.534 | 0.959 | 1.80× |
| 1,000 | 1 | 0.069 | 0.108 | 1.57× |
| 1,000 | 5 | 0.335 | 0.646 | 1.93× |
| 1,000 | 10 | 0.786 | 1.179 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
