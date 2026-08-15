# McClellanOscillator benchmark (`McClellanOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.94M | 0.005 | 187.95M | 8.270 | 1198.63× | 1554.32× |
| 10,000 | 0.051 | 194.81M | 0.046 | 215.72M | 83.047 | 1617.83× | 1791.54× |
| 100,000 | 0.459 | 217.96M | 0.434 | 230.32M | 826.439 | 1801.34× | 1903.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.255 | 2.61× |
| 1 | 5 | 0.298 | 1.391 | 4.66× |
| 1 | 10 | 0.408 | 2.192 | 5.38× |
| 10 | 1 | 0.050 | 0.285 | 5.67× |
| 10 | 5 | 0.181 | 1.713 | 9.47× |
| 10 | 10 | 0.431 | 2.928 | 6.79× |
| 100 | 1 | 0.051 | 1.056 | 20.89× |
| 100 | 5 | 0.221 | 5.690 | 25.70× |
| 100 | 10 | 0.427 | 11.107 | 25.99× |
| 1,000 | 1 | 0.063 | 8.716 | 138.52× |
| 1,000 | 5 | 0.353 | 46.260 | 131.18× |
| 1,000 | 10 | 0.535 | 93.374 | 174.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
