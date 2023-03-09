#pragma once

#include "annoylib.h"

#ifdef __cplusplus
extern "C" {
#endif

  AnnoyIndexInterface<int32_t, float>* annoy_index_angular(int f);

  void annoy_delete_index(AnnoyIndexInterface<int32_t, float> *index);

  void annoy_add_item(AnnoyIndexInterface<int32_t, float> *index, int item, float *w);

  void annoy_build(AnnoyIndexInterface<int32_t, float> *index, int q);

  void annoy_get_item(AnnoyIndexInterface<int32_t, float> *index, int item, float *result);

  void annoy_get_nns_by_item(AnnoyIndexInterface<int32_t, float> *index, int item, int n, int search_k, int *result, float *distances);

  void annoy_get_nns_by_vector(AnnoyIndexInterface<int32_t, float> *index, const float *w, int n, int search_k, int *result, float *distances);

  void annoy_load(AnnoyIndexInterface<int32_t, float> *index, const char *file);

  void annoy_save(AnnoyIndexInterface<int32_t, float> *index, const char *file);

  void annoy_set_seed(AnnoyIndexInterface<int32_t, float> *index, uint32_t q);

#ifdef __cplusplus
}
#endif