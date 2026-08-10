# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.62M | 0.010 | 98.45M | 0.033 | 2.50× | 3.30× |
| 10,000 | 0.076 | 132.16M | 0.072 | 139.33M | 0.102 | 1.35× | 1.43× |
| 100,000 | 0.749 | 133.57M | 0.737 | 135.73M | 0.737 | 0.98× | 1.00× |
| 1,000,000 | 7.624 | 131.17M | 7.370 | 135.68M | 7.394 | 0.97× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.108 | 1.08× |
| 1 | 5 | 0.328 | 0.444 | 1.35× |
| 1 | 10 | 0.514 | 0.895 | 1.74× |
| 10 | 1 | 0.053 | 0.085 | 1.61× |
| 10 | 5 | 0.240 | 0.411 | 1.71× |
| 10 | 10 | 0.553 | 0.903 | 1.63× |
| 100 | 1 | 0.057 | 0.094 | 1.65× |
| 100 | 5 | 0.254 | 0.427 | 1.68× |
| 100 | 10 | 0.531 | 0.934 | 1.76× |
| 1,000 | 1 | 0.070 | 0.104 | 1.47× |
| 1,000 | 5 | 0.256 | 0.472 | 1.84× |
| 1,000 | 10 | 0.580 | 0.990 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
