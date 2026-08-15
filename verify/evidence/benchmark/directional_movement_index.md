# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.82M | 0.012 | 80.75M | 0.043 | 2.72× | 3.45× |
| 10,000 | 0.120 | 83.45M | 0.113 | 88.24M | 0.125 | 1.04× | 1.10× |
| 100,000 | 1.242 | 80.49M | 1.156 | 86.51M | 0.896 | 0.72× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.138 | 1.42× |
| 1 | 5 | 0.196 | 0.481 | 2.45× |
| 1 | 10 | 0.388 | 0.972 | 2.51× |
| 10 | 1 | 0.044 | 0.096 | 2.20× |
| 10 | 5 | 0.230 | 0.494 | 2.15× |
| 10 | 10 | 0.476 | 0.962 | 2.02× |
| 100 | 1 | 0.040 | 0.094 | 2.32× |
| 100 | 5 | 0.193 | 0.452 | 2.34× |
| 100 | 10 | 0.429 | 0.940 | 2.19× |
| 1,000 | 1 | 0.052 | 0.108 | 2.07× |
| 1,000 | 5 | 0.192 | 0.480 | 2.49× |
| 1,000 | 10 | 0.431 | 1.073 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
