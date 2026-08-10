# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.49M | 0.061 | 16.49M | 0.091 | 1.50× | 1.50× |
| 10,000 | 0.565 | 17.71M | 0.563 | 17.75M | 0.624 | 1.10× | 1.11× |
| 100,000 | 5.795 | 17.26M | 5.651 | 17.70M | 5.500 | 0.95× | 0.97× |
| 1,000,000 | 73.188 | 13.66M | 57.750 | 17.32M | 55.182 | 0.75× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.127 | 1.19× |
| 1 | 5 | 0.352 | 0.552 | 1.57× |
| 1 | 10 | 0.450 | 1.069 | 2.37× |
| 10 | 1 | 0.060 | 0.108 | 1.79× |
| 10 | 5 | 0.250 | 0.500 | 2.00× |
| 10 | 10 | 0.443 | 1.030 | 2.32× |
| 100 | 1 | 0.057 | 0.105 | 1.84× |
| 100 | 5 | 0.264 | 0.568 | 2.15× |
| 100 | 10 | 0.524 | 1.103 | 2.10× |
| 1,000 | 1 | 0.111 | 0.153 | 1.38× |
| 1,000 | 5 | 0.229 | 0.831 | 3.62× |
| 1,000 | 10 | 0.595 | 1.625 | 2.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
