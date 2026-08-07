# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.083 | 12.06M | 0.040 | 25.03M | 0.050 | 0.60× | 1.25× |
| 10,000 | 0.868 | 11.52M | 0.446 | 22.44M | 0.136 | 0.16× | 0.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
