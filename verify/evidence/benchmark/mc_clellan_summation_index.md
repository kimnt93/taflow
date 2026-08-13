# McClellanSummationIndex benchmark (`McClellanSummationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.23M | 0.045 | 22.02M | 7.751 | 149.01× | 170.70× |
| 10,000 | 0.383 | 26.12M | 0.389 | 25.73M | 77.789 | 203.16× | 200.15× |
| 100,000 | 3.711 | 26.95M | 3.657 | 27.35M | 782.935 | 210.98× | 214.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.269 | 1.88× |
| 1 | 5 | 0.381 | 1.111 | 2.92× |
| 1 | 10 | 0.629 | 2.112 | 3.36× |
| 10 | 1 | 0.067 | 0.292 | 4.34× |
| 10 | 5 | 0.300 | 1.708 | 5.70× |
| 10 | 10 | 0.642 | 2.959 | 4.61× |
| 100 | 1 | 0.075 | 1.051 | 14.04× |
| 100 | 5 | 0.310 | 5.587 | 18.05× |
| 100 | 10 | 0.645 | 10.634 | 16.48× |
| 1,000 | 1 | 0.116 | 8.512 | 73.61× |
| 1,000 | 5 | 0.385 | 45.164 | 117.27× |
| 1,000 | 10 | 0.818 | 91.991 | 112.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
