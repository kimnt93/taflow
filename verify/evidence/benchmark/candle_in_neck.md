# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.69M | 0.003 | 341.19M | 0.034 | 5.54× | 11.76× |
| 10,000 | 0.068 | 148.09M | 0.063 | 159.14M | 0.118 | 1.75× | 1.88× |
| 100,000 | 0.809 | 123.53M | 0.774 | 129.24M | 0.969 | 1.20× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.110 | 1.72× |
| 1 | 5 | 0.269 | 0.450 | 1.67× |
| 1 | 10 | 0.397 | 0.915 | 2.30× |
| 10 | 1 | 0.041 | 0.090 | 2.23× |
| 10 | 5 | 0.194 | 0.437 | 2.25× |
| 10 | 10 | 0.422 | 1.020 | 2.42× |
| 100 | 1 | 0.045 | 0.089 | 1.98× |
| 100 | 5 | 0.188 | 0.442 | 2.35× |
| 100 | 10 | 0.406 | 0.962 | 2.37× |
| 1,000 | 1 | 0.053 | 0.095 | 1.79× |
| 1,000 | 5 | 0.203 | 0.511 | 2.52× |
| 1,000 | 10 | 0.452 | 1.047 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
