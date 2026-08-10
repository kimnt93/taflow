# CupAndHandle benchmark (`CupAndHandle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.44M | 0.010 | 99.86M | 0.217 | 17.27× | 21.71× |
| 10,000 | 0.085 | 118.30M | 0.082 | 122.43M | 1.577 | 18.66× | 19.31× |
| 100,000 | 0.802 | 124.76M | 0.794 | 125.92M | 12.430 | 15.51× | 15.65× |
| 1,000,000 | 8.367 | 119.52M | 8.162 | 122.52M | 121.498 | 14.52× | 14.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.207 | 1.80× |
| 1 | 5 | 0.418 | 1.120 | 2.68× |
| 1 | 10 | 0.504 | 1.643 | 3.26× |
| 10 | 1 | 0.052 | 0.173 | 3.33× |
| 10 | 5 | 0.245 | 1.074 | 4.38× |
| 10 | 10 | 0.533 | 1.660 | 3.11× |
| 100 | 1 | 0.052 | 0.180 | 3.44× |
| 100 | 5 | 0.259 | 1.230 | 4.74× |
| 100 | 10 | 0.600 | 2.016 | 3.36× |
| 1,000 | 1 | 0.064 | 0.302 | 4.70× |
| 1,000 | 5 | 0.288 | 1.854 | 6.45× |
| 1,000 | 10 | 0.611 | 3.468 | 5.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
