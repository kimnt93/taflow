# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.55M | 0.008 | 132.21M | 0.028 | 3.11× | 3.75× |
| 10,000 | 0.035 | 283.82M | 0.032 | 316.03M | 0.042 | 1.19× | 1.33× |
| 100,000 | 0.323 | 309.80M | 0.282 | 354.07M | 0.161 | 0.50× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.116 | 1.20× |
| 1 | 5 | 0.304 | 0.494 | 1.62× |
| 1 | 10 | 0.536 | 1.033 | 1.93× |
| 10 | 1 | 0.060 | 0.089 | 1.49× |
| 10 | 5 | 0.258 | 0.474 | 1.84× |
| 10 | 10 | 0.559 | 0.979 | 1.75× |
| 100 | 1 | 0.057 | 0.085 | 1.49× |
| 100 | 5 | 0.250 | 0.472 | 1.89× |
| 100 | 10 | 0.545 | 0.950 | 1.74× |
| 1,000 | 1 | 0.055 | 0.089 | 1.63× |
| 1,000 | 5 | 0.266 | 0.462 | 1.74× |
| 1,000 | 10 | 0.586 | 0.995 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
