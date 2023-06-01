#ifndef ATOLAB_CDDS_UTIL_H_
#define ATOLAB_CDDS_UTIL_H_


#include "dds/dds.h"
#include "dds/ddsi/ddsi_serdata.h"
#include "dds/ddsi/ddsi_radmin.h"

//#define CY_DEBUG_ON 1
#ifdef CY_DEBUG_ON
    #define CY_DEBUG(msg) printf(msg)
    #define CY_DEBUG_WA(fmt, ...) printf(fmt, __VA_ARGS__)
#else
    #define CY_DEBUG(msg)
    #define CY_DEBUG_WA(fmt, ...)
#endif

dds_entity_t cdds_create_blob_topic(dds_entity_t dp, char *topic_name, char* type_name, bool is_keyless);

#endif /* ATOLAB_CDDS_UTIL_H_ */
