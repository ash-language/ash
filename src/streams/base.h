#ifndef BASE_STREAM_H
#define BASE_STREAM_H

#include <stdbool.h>
#include <sys/queue.h>

#include "base_methods.h"


#define STREAM(struct_name, type_name, call_name) \
    typedef struct struct_name##StreamChunk { \
        type_name data; \
        STAILQ_ENTRY(struct_name##StreamChunk) entries; \
    } struct_name##StreamChunk; \
\
    typedef STAILQ_HEAD(struct_name##StreamHead, struct_name##StreamChunk) struct_name##StreamHead; \
\
    typedef struct struct_name##Stream { \
        struct_name##StreamHead head; \
    } struct_name##Stream; \
\
    struct_name##Stream* new_##call_name##_stream(); \
\
    typedef struct { \
        struct_name##Stream* stream; \
    } struct_name##StreamSource; \
\
    struct_name##StreamSource* new_##call_name##_stream_source(struct_name##Stream* stream); \
    void connect_##call_name##_stream_source(struct_name##Stream* stream, struct_name##StreamSource* source); \
    bool call_name##_stream_write(struct_name##StreamSource* source, type_name value); \
\
    typedef struct { \
        struct_name##Stream* stream; \
    } struct_name##StreamSink; \
\
    struct_name##StreamSink* new_##call_name##_stream_sink(struct_name##Stream* stream); \
    void connect_##call_name##_stream_sink(struct_name##Stream* stream, struct_name##StreamSink* sink); \
    type_name call_name##_stream_read(struct_name##StreamSink* sink);

#define STREAM_METHODS(struct_name, type_name, call_name) \
    new_stream(struct_name, call_name) \
    new_stream_source(struct_name, call_name) \
    connect_stream_source(struct_name, call_name) \
    stream_write(struct_name, type_name, call_name) \
    new_stream_sink(struct_name, call_name) \
    connect_stream_sink(struct_name, call_name) \
    stream_read(struct_name, type_name, call_name)

#define STREAM_EXPAND(struct_name, type_name, call_name) \
    STREAM(struct_name, type_name, call_name)

#define STREAM_METHODS_EXPAND(struct_name, type_name, call_name) \
    STREAM_METHODS(struct_name, type_name, call_name)

#endif