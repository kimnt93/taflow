# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.67M | 0.016 | 63.00M | 0.038 | 2.02× | 2.42× |
| 10,000 | 0.160 | 62.49M | 0.157 | 63.66M | 0.179 | 1.12× | 1.14× |
| 100,000 | 1.564 | 63.92M | 1.509 | 66.25M | 1.641 | 1.05× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.109 | 1.07× |
| 1 | 5 | 0.369 | 0.549 | 1.49× |
| 1 | 10 | 0.616 | 1.003 | 1.63× |
| 10 | 1 | 0.074 | 0.088 | 1.19× |
| 10 | 5 | 0.254 | 0.456 | 1.80× |
| 10 | 10 | 0.738 | 0.983 | 1.33× |
| 100 | 1 | 0.059 | 0.097 | 1.66× |
| 100 | 5 | 0.286 | 0.441 | 1.54× |
| 100 | 10 | 0.640 | 1.109 | 1.73× |
| 1,000 | 1 | 0.079 | 0.105 | 1.34× |
| 1,000 | 5 | 0.324 | 0.533 | 1.65× |
| 1,000 | 10 | 0.652 | 1.305 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
