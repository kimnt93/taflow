# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.76M | 0.019 | 52.15M | 0.036 | 1.70× | 1.86× |
| 10,000 | 0.211 | 47.42M | 0.203 | 49.21M | 0.126 | 0.60× | 0.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
