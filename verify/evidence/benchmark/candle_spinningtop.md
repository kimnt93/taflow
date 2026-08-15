# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.68M | 0.004 | 278.14M | 0.032 | 5.07× | 9.00× |
| 10,000 | 0.085 | 117.68M | 0.078 | 128.57M | 0.122 | 1.44× | 1.57× |
| 100,000 | 0.989 | 101.13M | 1.022 | 97.84M | 1.026 | 1.04× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.125 | 1.70× |
| 1 | 5 | 0.245 | 0.446 | 1.82× |
| 1 | 10 | 0.395 | 0.898 | 2.27× |
| 10 | 1 | 0.042 | 0.085 | 2.01× |
| 10 | 5 | 0.190 | 0.463 | 2.44× |
| 10 | 10 | 0.415 | 0.958 | 2.31× |
| 100 | 1 | 0.045 | 0.095 | 2.10× |
| 100 | 5 | 0.190 | 0.442 | 2.32× |
| 100 | 10 | 0.415 | 0.925 | 2.23× |
| 1,000 | 1 | 0.051 | 0.103 | 2.02× |
| 1,000 | 5 | 0.192 | 0.483 | 2.51× |
| 1,000 | 10 | 0.419 | 1.070 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
