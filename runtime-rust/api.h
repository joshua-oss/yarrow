/* Generated with cbindgen:0.9.1 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int64_t len;
  uint8_t *data;
} ByteBuffer;

ByteBuffer compute_release(const uint8_t *dataset_ptr,
                           int32_t dataset_length,
                           const uint8_t *analysis_ptr,
                           int32_t analysis_length,
                           const uint8_t *release_ptr,
                           int32_t release_length);
