# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.20M | 0.024 | 42.22M | 0.034 | 1.01× | 1.41× |
| 10,000 | 0.156 | 64.07M | 0.156 | 64.08M | 0.053 | 0.34× | 0.34× |
| 100,000 | 1.459 | 68.53M | 1.506 | 66.41M | 0.229 | 0.16× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.127 | 1.02× |
| 1 | 5 | 0.366 | 0.495 | 1.35× |
| 1 | 10 | 0.723 | 0.993 | 1.37× |
| 10 | 1 | 0.079 | 0.094 | 1.19× |
| 10 | 5 | 0.332 | 0.447 | 1.35× |
| 10 | 10 | 0.693 | 0.974 | 1.41× |
| 100 | 1 | 0.070 | 0.094 | 1.34× |
| 100 | 5 | 0.344 | 0.461 | 1.34× |
| 100 | 10 | 0.685 | 0.939 | 1.37× |
| 1,000 | 1 | 0.092 | 0.093 | 1.01× |
| 1,000 | 5 | 0.326 | 0.456 | 1.40× |
| 1,000 | 10 | 0.719 | 0.979 | 1.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
