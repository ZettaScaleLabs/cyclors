//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
#include <assert.h>
#include <limits.h>
#include <string.h>
#include "cdds/cdds_util.h"

struct cdds_ddsi_payload
{
  struct ddsi_serdata sd;
  size_t size;
  enum ddsi_serdata_kind kind;
  unsigned char *payload;
};

static bool cdds_sertype_equal(const struct ddsi_sertype *acmn, const struct ddsi_sertype *bcmn)
{
  // no fields in stp beyond the common ones, and those are all checked for equality before this function is called
  (void)acmn;
  (void)bcmn;
  return true;
}

static size_t get_hash(const char *source)
{
  size_t length = strlen(source);
  size_t hash = 0;
  for (size_t i = 0; i < length; i++)
  {
    char c = source[i];
    int a = c - '0';
    hash = (hash * 10) + a;
  }
  return hash;
}

static uint32_t cdds_sertype_hash(const struct ddsi_sertype *tpcmn)
{
  // nothing beyond the common fields
  (void)tpcmn;
  return get_hash(tpcmn->type_name);
}

static void cdds_sertype_free(struct ddsi_sertype *tpcmn)
{
  ddsi_sertype_fini(tpcmn);
  free(tpcmn);
}

static void cdds_sertype_zero_samples(const struct ddsi_sertype *d, void *samples, size_t count)
{
  (void)d;
  (void)samples;
  (void)count;
  /* Not using code paths that rely on the samples getting zero'd out */
}

static void cdds_sertype_realloc_samples(
    void **ptrs, const struct ddsi_sertype *d,
    void *old, size_t oldcount, size_t count)
{
  (void)(ptrs);
  (void)(d);
  (void)(old);
  (void)(oldcount);
  (void)(count);
  /* Not using code paths that rely on this (loans, dispose, unregister with instance handle,
     content filters) */
  abort();
}

static void cdds_sertype_free_samples(
    const struct ddsi_sertype *d, void **ptrs, size_t count,
    dds_free_op_t op)
{
  (void)(d);     // unused
  (void)(ptrs);  // unused
  (void)(count); // unused
  /* Not using code paths that rely on this (dispose, unregister with instance handle, content
     filters) */
  assert(!(op & DDS_FREE_ALL_BIT));
  (void)op;
}

static const struct ddsi_sertype_ops cdds_sertype_ops = {
    .version = ddsi_sertype_v0,
    .arg = NULL,
    .free = cdds_sertype_free,
    .zero_samples = cdds_sertype_zero_samples,
    .realloc_samples = cdds_sertype_realloc_samples,
    .free_samples = cdds_sertype_free_samples,
    .equal = cdds_sertype_equal,
    .hash = cdds_sertype_hash

    /* Here .type_id, .type_map, .type_info and .derive_sertype are needed if we want full XTypes across the bridge */
};

static bool cdds_serdata_eqkey(const struct ddsi_serdata *a, const struct ddsi_serdata *b)
{
  (void)(a);
  (void)(b);
  /* ROS 2 doesn't do keys in a meaningful way yet */
  CY_DEBUG("Called <cdds_serdata_eqkey>\n");
  return true;
}

static uint32_t cdds_serdata_size(const struct ddsi_serdata *sd)
{
  CY_DEBUG("Called <cdds_serdata_size>\n");
  struct cdds_ddsi_payload *zp = (struct cdds_ddsi_payload *)sd;
  CY_DEBUG_WA("Called <cdds_serdata_size> zp: %p\n", zp);
  assert(zp != 0);
  return zp->size;
}

static void cdds_serdata_free(struct ddsi_serdata *sd)
{
  CY_DEBUG("Called <cdds_serdata_free>\n");
  struct cdds_ddsi_payload *zp = (struct cdds_ddsi_payload *)sd;
  assert(zp != 0);
  // assert(zp->payload != 0);
  free(zp->payload);
  zp->payload = 0;
  zp->size = 0;
  free(zp);
}

static struct ddsi_serdata *cdds_serdata_from_ser_iov(const struct ddsi_sertype *tpcmn, enum ddsi_serdata_kind kind, ddsrt_msg_iovlen_t niov, const ddsrt_iovec_t *iov, size_t size)
{
  CY_DEBUG_WA("==> <cdds_serdata_from_ser_iov> for %s -- size %zu\n", tpcmn->name, size);
  struct cdds_ddsi_payload *zp = (struct cdds_ddsi_payload *)malloc(sizeof(struct cdds_ddsi_payload));
  ddsi_serdata_init(&zp->sd, tpcmn, kind);
  zp->size = size;
  zp->kind = kind;
  zp->payload = malloc(size);
  int offset = 0;
  int csize = 0;
  int i = 0;
  switch (kind)
  {
  case SDK_KEY:
  case SDK_DATA:
    for (i = 0; i < niov; ++i)
    {
      csize += iov[i].iov_len;
      assert(csize <= size);
      memcpy(zp->payload + offset, iov[i].iov_base, iov[i].iov_len);
      offset += iov[i].iov_len;
    }
    break;
  case SDK_EMPTY:
    break;
  }
  return (struct ddsi_serdata *)zp;
}

static struct ddsi_serdata *cdds_serdata_from_ser(
    const struct ddsi_sertype *tpcmn,
    enum ddsi_serdata_kind kind,
    const struct ddsi_rdata *fragchain, size_t size)
{
  CY_DEBUG_WA("Called <cdds_serdata_from_ser> for %s for %zu bytes\n", tpcmn->name, size);
  struct cdds_ddsi_payload *csd = (struct cdds_ddsi_payload *)malloc(sizeof(struct cdds_ddsi_payload));
  ddsi_serdata_init(&csd->sd, tpcmn, kind);
  csd->payload = (unsigned char *)malloc(size);
  csd->size = size;

  uint32_t off = 0;
  assert(fragchain->min == 0);
  assert(fragchain->maxp1 >= off);
  unsigned char *cursor = csd->payload;

  while (fragchain)
  {
    CY_DEBUG_WA("Defragmenting with offset: %d\n", off);
    if (fragchain->maxp1 > off)
    {
      const unsigned char *payload =
          DDSI_RMSG_PAYLOADOFF(fragchain->rmsg, DDSI_RDATA_PAYLOAD_OFF(fragchain));
      const unsigned char *src = payload + off - fragchain->min;
      uint32_t n_bytes = fragchain->maxp1 - off;
      CY_DEBUG_WA("Trying to memcpy %d bytes\n", n_bytes);
      memcpy(cursor, src, n_bytes);
      cursor = cursor + n_bytes;
      off = fragchain->maxp1;
      assert(off <= size);
    }
    fragchain = fragchain->nextfrag;
  }
  CY_DEBUG("Done Defragmenting!n");
  return &csd->sd;
}

static struct ddsi_serdata *cdds_serdata_to_typeless(const struct ddsi_serdata *psd)
{

  CY_DEBUG("Called <cdds_serdata_to_typeless> \n");
  struct cdds_ddsi_payload *sd = (struct cdds_ddsi_payload *)psd;
  struct cdds_ddsi_payload *sd_tl = (struct cdds_ddsi_payload *)malloc(sizeof(struct cdds_ddsi_payload));

  ddsi_serdata_init(&sd_tl->sd, sd->sd.type, SDK_KEY);
  sd_tl->sd.type = NULL;
  sd_tl->sd.hash = sd->sd.hash;
  sd_tl->sd.timestamp.v = INT64_MIN;
  sd_tl->payload = NULL;
  return &sd_tl->sd;
}

static struct ddsi_serdata *cdds_to_ser_ref(const struct ddsi_serdata *serdata_common, size_t cdr_off, size_t cdr_sz, ddsrt_iovec_t *ref)
{
  struct cdds_ddsi_payload *pl = (struct cdds_ddsi_payload *)serdata_common;
  CY_DEBUG("Called <cdds_to_ser_ref> \n");
  CY_DEBUG_WA("Called <cdds_to_ser_ref> offset = %zu\n", cdr_off);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> size = %zu\n", cdr_sz);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> ref = %p\n", ref);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> ref->iobase = %p\n", ref->iov_base);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> ref->iov_len = %zu\n", ref->iov_len);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> pl->payload = %p\n", pl->payload);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> pl->size = %zu\n", pl->size);

  ref->iov_base = pl->payload + cdr_off;
  uint8_t *buf = (uint8_t *)ref->iov_base;
  ref->iov_len = cdr_sz;
  return ddsi_serdata_ref(serdata_common);
}

static void cdds_to_ser_unref(struct ddsi_serdata *serdata_common, const ddsrt_iovec_t *ref)
{
  CY_DEBUG("Called <cdds_to_ser_unref> \n");
  CY_DEBUG_WA("Called <cdds_to_ser_ref> ref->iobase = %p\n", ref->iov_base);
  CY_DEBUG_WA("Called <cdds_to_ser_ref> ref->iov_len = %zu\n", ref->iov_len);
  (void)serdata_common;
  ddsi_serdata_unref(serdata_common);
}

static void cdds_to_ser(const struct ddsi_serdata *serdata_common, size_t off, size_t sz, void *buf)
{
  CY_DEBUG("Called <cdds_to_ser> \n");
  CY_DEBUG_WA("Called <cdds_to_ser> offset = %zu\n", off);
  CY_DEBUG_WA("Called <cdds_to_ser> size = %zu\n", sz);
  CY_DEBUG_WA("Called <cdds_to_ser> buf = %p\n", buf);
  struct cdds_ddsi_payload *pl = (struct cdds_ddsi_payload *)serdata_common;
  memcpy(buf, pl->payload, pl->size);
}

static const struct ddsi_serdata_ops cdds_serdata_ops = {
    .get_size = cdds_serdata_size,
    .eqkey = cdds_serdata_eqkey,
    .from_ser = cdds_serdata_from_ser,
    .from_ser_iov = cdds_serdata_from_ser_iov,
    .to_untyped = cdds_serdata_to_typeless,
    .to_ser = cdds_to_ser,
    .to_ser_ref = cdds_to_ser_ref,
    .to_ser_unref = cdds_to_ser_unref,
    .free = cdds_serdata_free};

dds_entity_t cdds_create_blob_topic(dds_entity_t dp, char *topic_name, char *type_name, bool is_keyless)
{
  CY_DEBUG("Called <cdds_create_blob_topic> \n");
  struct ddsi_sertype *st = (struct ddsi_sertype *)malloc(sizeof(struct ddsi_sertype));
  ddsi_sertype_init(st, type_name, &cdds_sertype_ops, &cdds_serdata_ops, is_keyless);
  return dds_create_topic_sertype(dp, topic_name, &st, NULL, NULL, NULL);
}
