# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.065 | 15.27M | 0.022 | 45.09M | 0.046 | 0.70× | 2.07× |
| 10,000 | 0.621 | 16.10M | 0.211 | 47.47M | 0.177 | 0.28× | 0.84× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.095 ms**; native kernel **0.035 ms**; TA-Lib 0.052 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.310 | 0.203 | 4.92M | 53.153 | 261.62× | 148.52× |
| 1,500 | 10 | 1.983 | 0.939 | 10.65M | 52.601 | 56.02× | 32.88× |
| 1,500 | 100 | 8.753 | 4.425 | 22.60M | 54.277 | 12.27× | 7.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
