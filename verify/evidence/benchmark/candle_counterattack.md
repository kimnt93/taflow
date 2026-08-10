# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.13M | 0.017 | 57.99M | 0.039 | 1.90× | 2.28× |
| 10,000 | 0.182 | 55.08M | 0.179 | 55.86M | 0.141 | 0.77× | 0.79× |
| 100,000 | 1.859 | 53.78M | 1.831 | 54.62M | 1.194 | 0.64× | 0.65× |
| 1,000,000 | 18.895 | 52.92M | 18.150 | 55.10M | 12.860 | 0.68× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.113 | 1.25× |
| 1 | 5 | 0.341 | 0.469 | 1.38× |
| 1 | 10 | 0.564 | 0.993 | 1.76× |
| 10 | 1 | 0.061 | 0.088 | 1.44× |
| 10 | 5 | 0.277 | 0.483 | 1.75× |
| 10 | 10 | 0.547 | 0.943 | 1.72× |
| 100 | 1 | 0.055 | 0.111 | 2.00× |
| 100 | 5 | 0.286 | 0.482 | 1.68× |
| 100 | 10 | 0.653 | 0.950 | 1.45× |
| 1,000 | 1 | 0.073 | 0.097 | 1.34× |
| 1,000 | 5 | 0.305 | 0.532 | 1.74× |
| 1,000 | 10 | 0.586 | 1.102 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
