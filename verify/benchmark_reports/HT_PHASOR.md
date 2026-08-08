# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.78M | 0.049 | 20.32M | 0.078 | 1.54× | 1.58× |
| 10,000 | 0.478 | 20.93M | 0.477 | 20.98M | 0.500 | 1.05× | 1.05× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.075 ms**; native kernel **0.076 ms**; TA-Lib 0.104 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.242 | 4.13M | 97.098 | 400.80× | 137.18× |
| 1,500 | 10 | 1.526 | 0.958 | 10.44M | 99.327 | 103.67× | 33.32× |
| 1,500 | 100 | 8.087 | 6.347 | 15.76M | 111.103 | 17.50× | 6.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
