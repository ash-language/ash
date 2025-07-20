#ifndef BASE_STREAM_METHODS_H
#define BASE_STREAM_METHODS_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <sys/queue.h>


#define new_stream(struct_name, call_name) \
    struct_name##Stream* new_##call_name##_stream() { \
        struct_name##Stream* stream = malloc(sizeof(struct_name##Stream)); \
    \
        if (stream == NULL) { \
            return NULL; \
        } \
    \
        STAILQ_INIT(&stream->head); \
    \
        return stream; \
    }

#define new_stream_source(struct_name, call_name) \
    struct_name##StreamSource* new_##call_name##_stream_source(struct_name##Stream* stream) { \
        struct_name##StreamSource* source = malloc(sizeof(struct_name##StreamSource)); \
    \
        if (source == NULL) { return NULL; } \
    \
        source->stream = stream; \
    \
        return source; \
    }

#define connect_stream_source(struct_name, call_name) \
    void connect_##call_name##_stream_source(struct_name##Stream* stream, struct_name##StreamSource* source) { \
        source->stream = stream; \
    }

#define stream_write(struct_name, type_name, call_name) \
    bool call_name##_stream_write(struct_name##StreamSource* source, type_name data) { \
        struct_name##StreamChunk* chunk = malloc(sizeof(struct_name##StreamChunk)); \
    \
        if (chunk == NULL) { return false; } \
    \
        chunk->data = data; \
        struct_name##Stream* stream = source->stream; \
    \
        STAILQ_INSERT_TAIL(&stream->head, chunk, entries); \
    \
        return true; \
    }

#define new_stream_sink(struct_name, call_name) \
    struct_name##StreamSink* new_##call_name##_stream_sink(struct_name##Stream* stream) { \
        struct_name##StreamSink* source = malloc(sizeof(struct_name##StreamSink)); \
    \
        if (source == NULL) { return NULL; } \
    \
        source->stream = stream; \
    \
        return source; \
    }

#define connect_stream_sink(struct_name, call_name) \
    void connect_##call_name##_stream_sink(struct_name##Stream* stream, struct_name##StreamSink* sink) { \
        sink->stream = stream; \
    }

#define stream_read(struct_name, type_name, call_name) \
    type_name call_name##_stream_read(struct_name##StreamSink* sink) { \
        struct_name##Stream* stream = sink->stream; \
        struct_name##StreamHead* head = &stream->head; \
    \
        if (STAILQ_EMPTY(head)) { return EOF; } \
    \
        struct_name##StreamChunk* np = STAILQ_FIRST(head); \
    \
        if (np == NULL) { return EOF; } \
    \
        return np->data; \
    }

#endif