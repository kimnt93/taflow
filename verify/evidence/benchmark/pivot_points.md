# PivotPoints benchmark (`anchored classic pivot points` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.76M | 0.070 | 14.26M | 0.879 | 10.34× | 12.53× |
| 10,000 | 0.596 | 16.77M | 0.564 | 17.74M | 8.824 | 14.80× | 15.66× |
| 100,000 | 5.637 | 17.74M | 5.330 | 18.76M | 85.410 | 15.15× | 16.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.110 | 0.74× |
| 1 | 5 | 0.384 | 0.376 | 0.98× |
| 1 | 10 | 0.662 | 0.703 | 1.06× |
| 10 | 1 | 0.071 | 0.098 | 1.38× |
| 10 | 5 | 0.323 | 0.389 | 1.21× |
| 10 | 10 | 0.678 | 0.849 | 1.25× |
| 100 | 1 | 0.083 | 0.166 | 2.00× |
| 100 | 5 | 0.370 | 0.841 | 2.28× |
| 100 | 10 | 0.736 | 1.665 | 2.26× |
| 1,000 | 1 | 0.137 | 0.985 | 7.16× |
| 1,000 | 5 | 0.633 | 5.153 | 8.14× |
| 1,000 | 10 | 1.270 | 10.343 | 8.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
