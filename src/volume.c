#include <assert.h>
#include <pulse/pulseaudio.h>
#include <signal.h>
#include <stdio.h>
#include <pthread.h>
float f = 0.0;
int muted = 0;

int thread_status = -1;

pa_mainloop *mainloop;
pa_mainloop_api *mainloop_api;
float get_vol() { return f; }
int get_muted() { return muted; }
int get_thread_status() { return thread_status; }
void exit_signal_callback(pa_mainloop_api *m, pa_signal_event *e, int n, void* v) {
    printf("exit\n");
    thread_status = 0;
    mainloop_api->quit(mainloop_api, 0);
}
void sink_info_callback(pa_context *c, const pa_sink_info *i,
            int eol, void *userdata)
    {
        if (i)
        {
            float volume = (float)pa_cvolume_avg(&(i->volume)) / (float)PA_VOLUME_NORM;
            muted = i->mute;
            f = volume * 100.0f;
            // printf("percent volume = %.0f%%%s\n", volume * 100.0f, i->mute ? " (muted)" : "");
            // mainloop_api->quit(mainloop_api, 0);
        }
    }

void server_info_callback(pa_context *c, const pa_server_info *i,
                          void *userdata) {
    pa_context_get_sink_info_by_name(c, i->default_sink_name, sink_info_callback, userdata);
}
void subscribe_callback(pa_context *c,
            pa_subscription_event_type_t type, uint32_t idx, void *userdata)
    {
        pa_operation *op = pa_context_get_sink_info_by_index(c, idx, sink_info_callback, userdata);;
        if (op) pa_operation_unref(op);
    }
void context_state_callback(pa_context *c, void *e) {
    switch (pa_context_get_state(c)) {
    case PA_CONTEXT_READY:
        pa_context_get_server_info(c, server_info_callback, e);
        pa_context_set_subscribe_callback(c, subscribe_callback, e);
        pa_context_subscribe(c, PA_SUBSCRIPTION_MASK_SINK, NULL, NULL);
        break;
    case PA_CONTEXT_TERMINATED:
        mainloop_api->quit(mainloop_api, 0);
        break;
    case PA_CONTEXT_FAILED:
    default:
        break;
    }
}
void *run() {
    thread_status = 1;
    pa_mainloop_run(mainloop, NULL);
    return NULL;
}
void vol_thread_start() {
    mainloop = pa_mainloop_new();
    mainloop_api = pa_mainloop_get_api(mainloop);
    pa_signal_init(mainloop_api);
    pa_signal_event *psignal = pa_signal_new(SIGINT, exit_signal_callback, mainloop_api);
    pa_context *context = pa_context_new(mainloop_api, "PA");
    pa_context_connect(context, NULL, PA_CONTEXT_NOAUTOSPAWN, NULL);
    pa_context_set_state_callback(context, context_state_callback, NULL);
    
    pthread_t thread_id;
    pthread_create(&thread_id, NULL, run, NULL);
}

