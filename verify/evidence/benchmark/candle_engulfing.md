# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.95M | 0.003 | 293.75M | 0.032 | 4.75× | 9.31× |
| 10,000 | 0.065 | 154.04M | 0.062 | 162.04M | 0.085 | 1.31× | 1.38× |
| 100,000 | 0.788 | 126.91M | 0.729 | 137.22M | 0.559 | 0.71× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.191 | 0.126 | 0.66× |
| 1 | 5 | 0.389 | 0.472 | 1.21× |
| 1 | 10 | 0.396 | 0.899 | 2.27× |
| 10 | 1 | 0.047 | 0.094 | 2.00× |
| 10 | 5 | 0.194 | 0.452 | 2.33× |
| 10 | 10 | 0.397 | 0.908 | 2.29× |
| 100 | 1 | 0.042 | 0.089 | 2.11× |
| 100 | 5 | 0.195 | 0.419 | 2.15× |
| 100 | 10 | 0.419 | 0.939 | 2.24× |
| 1,000 | 1 | 0.050 | 0.098 | 1.96× |
| 1,000 | 5 | 0.198 | 0.463 | 2.34× |
| 1,000 | 10 | 0.430 | 1.057 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
