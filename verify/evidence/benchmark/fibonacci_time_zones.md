# FibonacciTimeZones benchmark (`FibTimeZones` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.81M | 0.080 | 12.54M | 0.457 | 5.40× | 5.73× |
| 10,000 | 0.722 | 13.85M | 0.716 | 13.97M | 3.557 | 4.93× | 4.97× |
| 100,000 | 7.121 | 14.04M | 7.053 | 14.18M | 39.247 | 5.51× | 5.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.240 | 2.29× |
| 1 | 5 | 0.339 | 0.835 | 2.46× |
| 1 | 10 | 0.604 | 1.839 | 3.05× |
| 10 | 1 | 0.076 | 0.175 | 2.30× |
| 10 | 5 | 0.316 | 0.850 | 2.69× |
| 10 | 10 | 0.618 | 1.956 | 3.16× |
| 100 | 1 | 0.092 | 0.214 | 2.33× |
| 100 | 5 | 0.313 | 1.029 | 3.29× |
| 100 | 10 | 0.662 | 2.261 | 3.42× |
| 1,000 | 1 | 0.143 | 0.766 | 5.35× |
| 1,000 | 5 | 0.314 | 2.930 | 9.34× |
| 1,000 | 10 | 0.648 | 5.927 | 9.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
