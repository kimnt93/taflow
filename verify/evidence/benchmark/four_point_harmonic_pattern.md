# FourPointHarmonicPattern benchmark (`Abcd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.46M | 0.006 | 159.69M | 0.229 | 23.97× | 36.64× |
| 10,000 | 0.093 | 107.96M | 0.088 | 113.05M | 1.676 | 18.10× | 18.95× |
| 100,000 | 0.896 | 111.64M | 0.855 | 116.92M | 13.150 | 14.68× | 15.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.205 | 3.44× |
| 1 | 5 | 0.244 | 0.809 | 3.32× |
| 1 | 10 | 0.407 | 1.718 | 4.22× |
| 10 | 1 | 0.047 | 0.163 | 3.51× |
| 10 | 5 | 0.205 | 1.079 | 5.27× |
| 10 | 10 | 0.443 | 1.747 | 3.94× |
| 100 | 1 | 0.046 | 0.177 | 3.88× |
| 100 | 5 | 0.210 | 1.205 | 5.75× |
| 100 | 10 | 0.413 | 1.815 | 4.40× |
| 1,000 | 1 | 0.052 | 0.289 | 5.55× |
| 1,000 | 5 | 0.201 | 1.774 | 8.83× |
| 1,000 | 10 | 0.455 | 2.993 | 6.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
