# RollingMode benchmark (`rolling mode` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.214 | 4.67M | 0.226 | 4.42M | 0.046 | 0.21× | 0.20× |
| 10,000 | 2.141 | 4.67M | 2.382 | 4.20M | 0.131 | 0.06× | 0.05× |
| 100,000 | 21.251 | 4.71M | 22.111 | 4.52M | 0.947 | 0.04× | 0.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.125 | 0.93× |
| 1 | 5 | 0.250 | 0.526 | 2.11× |
| 1 | 10 | 0.423 | 0.960 | 2.27× |
| 10 | 1 | 0.050 | 0.083 | 1.67× |
| 10 | 5 | 0.186 | 0.435 | 2.34× |
| 10 | 10 | 0.466 | 0.919 | 1.97× |
| 100 | 1 | 0.066 | 0.126 | 1.91× |
| 100 | 5 | 0.221 | 0.537 | 2.43× |
| 100 | 10 | 0.517 | 1.480 | 2.86× |
| 1,000 | 1 | 0.268 | 0.123 | 0.46× |
| 1,000 | 5 | 0.552 | 0.722 | 1.31× |
| 1,000 | 10 | 0.729 | 1.564 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
