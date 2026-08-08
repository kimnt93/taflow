# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.102 | 9.84M | 0.099 | 10.06M | 0.470 | 4.63× | 4.73× |
| 10,000 | 1.084 | 9.22M | 1.082 | 9.24M | 4.561 | 4.21× | 4.22× |
| 100,000 | 11.415 | 8.76M | 10.411 | 9.61M | 47.062 | 4.12× | 4.52× |
| 1,000,000 | 106.684 | 9.37M | 100.539 | 9.95M | 440.731 | 4.13× | 4.38× |

## Warm-up

Construct + canonical extend over 100,000 bars: **10.085 ms**; native kernel **9.864 ms**; TA-Lib 43.369 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.373 | 0.281 | 3.55M | 46687.160 | 165911.99× | 114.84× |
| 100,000 | 10 | 1.981 | 1.606 | 6.23M | 42283.584 | 26330.66× | 23.00× |
| 100,000 | 1,000 | 114.988 | 97.523 | 10.25M | 42971.092 | 440.62× | 4.75× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.77M | 9.88M | 1.00× | 1.97M | 2.37M | 1.00× | 2.35M |
| 2 | 17.74M | 18.12M | 1.83× | 2.06M | 2.22M | 0.93× | 2.24M |
| 4 | 34.47M | 35.19M | 3.56× | 2.10M | 2.23M | 0.94× | 2.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
