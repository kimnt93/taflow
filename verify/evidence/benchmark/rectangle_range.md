# RectangleRange benchmark (`RectangleRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.04M | 0.013 | 75.36M | 0.235 | 16.45× | 17.70× |
| 10,000 | 0.114 | 87.99M | 0.142 | 70.44M | 1.700 | 14.96× | 11.98× |
| 100,000 | 0.877 | 114.03M | 0.874 | 114.38M | 12.090 | 13.79× | 13.83× |
| 1,000,000 | 9.341 | 107.05M | 9.203 | 108.67M | 122.885 | 13.16× | 13.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.210 | 2.27× |
| 1 | 5 | 0.300 | 1.095 | 3.65× |
| 1 | 10 | 0.579 | 1.799 | 3.11× |
| 10 | 1 | 0.053 | 0.166 | 3.11× |
| 10 | 5 | 0.271 | 1.139 | 4.21× |
| 10 | 10 | 0.551 | 1.794 | 3.26× |
| 100 | 1 | 0.066 | 0.194 | 2.96× |
| 100 | 5 | 0.307 | 1.304 | 4.25× |
| 100 | 10 | 0.580 | 1.837 | 3.17× |
| 1,000 | 1 | 0.062 | 0.356 | 5.70× |
| 1,000 | 5 | 0.262 | 1.741 | 6.64× |
| 1,000 | 10 | 0.545 | 2.976 | 5.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
