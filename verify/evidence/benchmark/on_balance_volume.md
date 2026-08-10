# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.42M | 0.010 | 104.58M | 0.038 | 3.66× | 3.92× |
| 10,000 | 0.074 | 135.74M | 0.071 | 140.90M | 0.075 | 1.02× | 1.06× |
| 100,000 | 0.711 | 140.72M | 0.663 | 150.84M | 0.455 | 0.64× | 0.69× |
| 1,000,000 | 7.658 | 130.59M | 7.205 | 138.79M | 4.275 | 0.56× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.110 | 1.43× |
| 1 | 5 | 0.308 | 0.522 | 1.70× |
| 1 | 10 | 0.567 | 1.149 | 2.02× |
| 10 | 1 | 0.055 | 0.095 | 1.73× |
| 10 | 5 | 0.268 | 0.551 | 2.06× |
| 10 | 10 | 0.611 | 1.086 | 1.78× |
| 100 | 1 | 0.051 | 0.085 | 1.68× |
| 100 | 5 | 0.267 | 0.474 | 1.77× |
| 100 | 10 | 0.536 | 1.193 | 2.23× |
| 1,000 | 1 | 0.063 | 0.099 | 1.56× |
| 1,000 | 5 | 0.289 | 0.590 | 2.04× |
| 1,000 | 10 | 0.576 | 1.380 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
