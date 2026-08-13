# GarmanKlassYangZhang benchmark (`annualized Garman-Klass-Yang-Zhang volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.116 | 8.60M | 0.107 | 9.33M | 0.119 | 1.02× | 1.11× |
| 10,000 | 0.961 | 10.41M | 0.930 | 10.76M | 0.442 | 0.46× | 0.48× |
| 100,000 | 9.421 | 10.61M | 9.732 | 10.28M | 3.548 | 0.38× | 0.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.129 | 1.07× |
| 1 | 5 | 0.452 | 0.764 | 1.69× |
| 1 | 10 | 0.676 | 1.161 | 1.72× |
| 10 | 1 | 0.075 | 0.123 | 1.64× |
| 10 | 5 | 0.324 | 0.603 | 1.86× |
| 10 | 10 | 0.660 | 1.168 | 1.77× |
| 100 | 1 | 0.079 | 0.158 | 2.01× |
| 100 | 5 | 0.313 | 0.769 | 2.46× |
| 100 | 10 | 0.678 | 1.572 | 2.32× |
| 1,000 | 1 | 0.174 | 0.192 | 1.11× |
| 1,000 | 5 | 0.392 | 1.064 | 2.71× |
| 1,000 | 10 | 0.683 | 2.235 | 3.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
