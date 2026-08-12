# PositiveVolumeIndex benchmark (`PVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.49M | 0.012 | 80.03M | 0.277 | 27.52× | 22.14× |
| 10,000 | 0.070 | 142.47M | 0.062 | 162.11M | 0.854 | 12.16× | 13.84× |
| 100,000 | 0.620 | 161.16M | 0.588 | 170.06M | 6.999 | 11.28× | 11.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.273 | 3.20× |
| 1 | 5 | 0.342 | 1.095 | 3.21× |
| 1 | 10 | 0.527 | 2.342 | 4.44× |
| 10 | 1 | 0.059 | 0.232 | 3.97× |
| 10 | 5 | 0.244 | 1.431 | 5.85× |
| 10 | 10 | 0.561 | 2.397 | 4.27× |
| 100 | 1 | 0.053 | 0.208 | 3.92× |
| 100 | 5 | 0.247 | 1.403 | 5.67× |
| 100 | 10 | 0.541 | 2.380 | 4.40× |
| 1,000 | 1 | 0.059 | 0.278 | 4.67× |
| 1,000 | 5 | 0.304 | 1.720 | 5.66× |
| 1,000 | 10 | 0.550 | 3.075 | 5.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
