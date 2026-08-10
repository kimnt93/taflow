# DoubleBollingerBands benchmark (`DoubleBollinger` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.94M | 0.044 | 22.78M | 0.721 | 15.82× | 16.43× |
| 10,000 | 0.411 | 24.34M | 0.383 | 26.10M | 4.632 | 11.27× | 12.09× |
| 100,000 | 3.941 | 25.37M | 3.645 | 27.44M | 47.027 | 11.93× | 12.90× |
| 1,000,000 | 42.171 | 23.71M | 39.410 | 25.37M | 548.006 | 13.00× | 13.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.307 | 3.49× |
| 1 | 5 | 0.304 | 1.427 | 4.70× |
| 1 | 10 | 0.499 | 2.898 | 5.81× |
| 10 | 1 | 0.059 | 0.267 | 4.52× |
| 10 | 5 | 0.278 | 1.432 | 5.15× |
| 10 | 10 | 0.522 | 2.989 | 5.73× |
| 100 | 1 | 0.058 | 0.301 | 5.20× |
| 100 | 5 | 0.255 | 1.602 | 6.29× |
| 100 | 10 | 0.525 | 3.308 | 6.30× |
| 1,000 | 1 | 0.100 | 0.831 | 8.31× |
| 1,000 | 5 | 0.284 | 3.957 | 13.92× |
| 1,000 | 10 | 0.566 | 7.769 | 13.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
