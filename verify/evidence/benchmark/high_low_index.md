# HighLowIndex benchmark (`HighLowIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.25M | 0.053 | 19.02M | 7.912 | 128.53× | 150.49× |
| 10,000 | 0.491 | 20.36M | 0.442 | 22.62M | 79.571 | 161.97× | 179.96× |
| 100,000 | 4.554 | 21.96M | 4.350 | 22.99M | 814.130 | 178.79× | 187.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.210 | 0.331 | 1.57× |
| 1 | 5 | 0.478 | 1.238 | 2.59× |
| 1 | 10 | 0.633 | 2.553 | 4.04× |
| 10 | 1 | 0.071 | 0.310 | 4.34× |
| 10 | 5 | 0.302 | 1.538 | 5.10× |
| 10 | 10 | 0.600 | 3.291 | 5.48× |
| 100 | 1 | 0.074 | 1.086 | 14.67× |
| 100 | 5 | 0.293 | 5.298 | 18.09× |
| 100 | 10 | 0.657 | 11.175 | 17.02× |
| 1,000 | 1 | 0.131 | 8.684 | 66.08× |
| 1,000 | 5 | 0.458 | 47.974 | 104.68× |
| 1,000 | 10 | 0.791 | 93.852 | 118.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
