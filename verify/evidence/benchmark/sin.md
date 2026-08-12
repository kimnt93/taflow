# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.87M | 0.012 | 84.57M | 0.040 | 2.98× | 3.42× |
| 10,000 | 0.157 | 63.61M | 0.148 | 67.53M | 0.183 | 1.16× | 1.24× |
| 100,000 | 1.527 | 65.51M | 1.534 | 65.18M | 1.536 | 1.01× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.091 | 0.88× |
| 1 | 5 | 0.311 | 0.456 | 1.46× |
| 1 | 10 | 0.494 | 0.923 | 1.87× |
| 10 | 1 | 0.047 | 0.095 | 2.02× |
| 10 | 5 | 0.277 | 0.459 | 1.66× |
| 10 | 10 | 0.477 | 0.942 | 1.97× |
| 100 | 1 | 0.050 | 0.093 | 1.86× |
| 100 | 5 | 0.236 | 0.452 | 1.92× |
| 100 | 10 | 0.566 | 0.924 | 1.63× |
| 1,000 | 1 | 0.065 | 0.103 | 1.58× |
| 1,000 | 5 | 0.250 | 0.513 | 2.05× |
| 1,000 | 10 | 0.544 | 1.171 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
