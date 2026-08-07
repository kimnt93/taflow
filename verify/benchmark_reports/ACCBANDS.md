# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.209 | 4.78M | 0.021 | 47.69M | 0.049 | 0.23× | 2.33× |
| 10,000 | 2.164 | 4.62M | 0.185 | 54.18M | 0.121 | 0.06× | 0.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
