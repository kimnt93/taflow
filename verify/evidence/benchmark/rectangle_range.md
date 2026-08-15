# RectangleRange benchmark (`RectangleRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.67M | 0.008 | 123.61M | 0.229 | 21.22× | 28.31× |
| 10,000 | 0.095 | 105.23M | 0.090 | 111.41M | 1.408 | 14.82× | 15.69× |
| 100,000 | 0.915 | 109.28M | 0.874 | 114.39M | 13.990 | 15.29× | 16.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.240 | 2.68× |
| 1 | 5 | 0.287 | 0.824 | 2.87× |
| 1 | 10 | 0.431 | 1.679 | 3.90× |
| 10 | 1 | 0.053 | 0.161 | 3.04× |
| 10 | 5 | 0.204 | 1.084 | 5.32× |
| 10 | 10 | 0.449 | 1.683 | 3.75× |
| 100 | 1 | 0.051 | 0.174 | 3.42× |
| 100 | 5 | 0.200 | 1.146 | 5.74× |
| 100 | 10 | 0.446 | 1.823 | 4.09× |
| 1,000 | 1 | 0.057 | 0.304 | 5.33× |
| 1,000 | 5 | 0.190 | 1.867 | 9.81× |
| 1,000 | 10 | 0.434 | 2.962 | 6.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
