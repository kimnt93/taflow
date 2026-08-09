# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 195.40M | 0.004 | 242.28M | 0.033 | 6.45× | 8.00× |
| 10,000 | 0.033 | 301.20M | 0.031 | 325.58M | 0.061 | 1.83× | 1.97× |
| 100,000 | 0.317 | 315.62M | 0.291 | 344.21M | 0.311 | 0.98× | 1.07× |
| 1,000,000 | 3.337 | 299.67M | 2.941 | 339.97M | 2.908 | 0.87× | 0.99× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.310 ms**; native kernel **0.289 ms**; TA-Lib 0.306 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.241 | 0.148 | 6.76M | 310.438 | 2099.28× | 200.98× |
| 100,000 | 10 | 0.930 | 0.555 | 18.01M | 317.409 | 571.64× | 54.54× |
| 100,000 | 1,000 | 5.584 | 4.425 | 226.01M | 311.791 | 70.47× | 7.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 204.83M | 238.53M | 1.00× | 2.82M | 2.81M | 1.00× | 246.29M |
| 2 | 424.22M | 511.51M | 2.14× | 2.97M | 4.18M | 1.49× | 231.74M |
| 4 | 609.91M | 913.45M | 3.83× | 3.06M | 3.25M | 1.16× | 237.01M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
