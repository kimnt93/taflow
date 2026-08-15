# StandardErrorBands benchmark (`StandardErrorBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.86M | 0.072 | 13.97M | 0.600 | 8.32× | 8.38× |
| 10,000 | 0.704 | 14.20M | 0.703 | 14.22M | 4.223 | 6.00× | 6.01× |
| 100,000 | 6.878 | 14.54M | 6.898 | 14.50M | 45.537 | 6.62× | 6.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.288 | 4.09× |
| 1 | 5 | 0.265 | 1.390 | 5.25× |
| 1 | 10 | 0.415 | 2.621 | 6.32× |
| 10 | 1 | 0.048 | 0.250 | 5.16× |
| 10 | 5 | 0.195 | 1.438 | 7.39× |
| 10 | 10 | 0.402 | 2.796 | 6.96× |
| 100 | 1 | 0.053 | 0.296 | 5.60× |
| 100 | 5 | 0.202 | 1.654 | 8.20× |
| 100 | 10 | 0.457 | 3.068 | 6.71× |
| 1,000 | 1 | 0.123 | 0.891 | 7.25× |
| 1,000 | 5 | 0.271 | 10.452 | 38.63× |
| 1,000 | 10 | 0.496 | 7.578 | 15.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
