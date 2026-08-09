# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.47M | 0.008 | 125.64M | 0.033 | 3.38× | 4.11× |
| 10,000 | 0.069 | 143.95M | 0.067 | 149.96M | 0.107 | 1.54× | 1.61× |
| 100,000 | 0.808 | 123.69M | 0.812 | 123.18M | 0.760 | 0.94× | 0.94× |
| 1,000,000 | 7.896 | 126.65M | 8.502 | 117.62M | 8.663 | 1.10× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.153 | 1.81× |
| 1 | 5 | 0.366 | 0.614 | 1.68× |
| 1 | 10 | 0.677 | 1.040 | 1.54× |
| 10 | 1 | 0.057 | 0.103 | 1.81× |
| 10 | 5 | 0.260 | 0.446 | 1.71× |
| 10 | 10 | 0.655 | 1.272 | 1.94× |
| 100 | 1 | 0.067 | 0.104 | 1.54× |
| 100 | 5 | 0.308 | 0.505 | 1.64× |
| 100 | 10 | 0.616 | 1.039 | 1.69× |
| 1,000 | 1 | 0.069 | 0.101 | 1.47× |
| 1,000 | 5 | 0.265 | 0.485 | 1.83× |
| 1,000 | 10 | 0.555 | 1.022 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
