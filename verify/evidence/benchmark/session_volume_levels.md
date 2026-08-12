# SessionVolumeLevels benchmark (`anchored volume levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.73M | 0.053 | 19.03M | 13.425 | 238.09× | 255.48× |
| 10,000 | 0.485 | 20.60M | 0.494 | 20.22M | 140.459 | 289.38× | 284.04× |
| 100,000 | 5.719 | 17.48M | 4.969 | 20.13M | 1388.320 | 242.74× | 279.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.146 | 1.36× |
| 1 | 5 | 0.341 | 0.853 | 2.50× |
| 1 | 10 | 0.523 | 1.403 | 2.69× |
| 10 | 1 | 0.055 | 0.307 | 5.58× |
| 10 | 5 | 0.260 | 1.635 | 6.29× |
| 10 | 10 | 0.514 | 3.246 | 6.31× |
| 100 | 1 | 0.071 | 1.825 | 25.67× |
| 100 | 5 | 0.279 | 9.865 | 35.42× |
| 100 | 10 | 0.553 | 20.557 | 37.15× |
| 1,000 | 1 | 0.115 | 13.905 | 120.99× |
| 1,000 | 5 | 0.719 | 76.652 | 106.65× |
| 1,000 | 10 | 1.085 | 159.193 | 146.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
