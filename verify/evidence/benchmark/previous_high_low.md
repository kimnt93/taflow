# PreviousHighLow benchmark (`previous-session high-low` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.98M | 0.015 | 67.07M | 0.607 | 35.21× | 40.73× |
| 10,000 | 0.113 | 88.49M | 0.099 | 100.55M | 6.219 | 55.03× | 62.53× |
| 100,000 | 1.042 | 95.94M | 0.956 | 104.55M | 57.583 | 55.24× | 60.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.095 | 1.37× |
| 1 | 5 | 0.240 | 0.331 | 1.38× |
| 1 | 10 | 0.510 | 0.681 | 1.34× |
| 10 | 1 | 0.050 | 0.083 | 1.66× |
| 10 | 5 | 0.230 | 0.359 | 1.56× |
| 10 | 10 | 0.514 | 0.769 | 1.50× |
| 100 | 1 | 0.057 | 0.122 | 2.15× |
| 100 | 5 | 0.260 | 0.625 | 2.41× |
| 100 | 10 | 0.533 | 1.256 | 2.36× |
| 1,000 | 1 | 0.063 | 0.674 | 10.75× |
| 1,000 | 5 | 0.252 | 3.344 | 13.29× |
| 1,000 | 10 | 0.557 | 6.797 | 12.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
